use crate::pieces::pawn::Pawn;
use crate::pieces::king::King;
use crate::pieces::queen::Queen;
use crate::pieces::bishop::Bishop;
use crate::pieces::knight::Knight;
use crate::pieces::rook::Rook;
use crate::pieces::piece::Piece;

pub struct Game {
    pub board: Vec<Vec<&'static str>>,
    pub turn: char,
    pub pieces: Vec<Box<dyn Piece>>,
    pub selected_figure_index: Option<i8>,
    pub valid_moves: Vec<(i8, i8)>,
    pub history: Vec<String>,
    pub captured_by_white: Vec<&'static str>,
    pub captured_by_black: Vec<&'static str>,
    pub en_passant_target: Option<(i8, i8)>,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub promotion: Option<usize>, // Store index of the pawn to promote
}

impl Game {
    pub fn new() -> Self {
        let mut board = vec![vec![""; 8]; 8];
        let mut pieces: Vec<Box<dyn Piece>> = Vec::new();

        // Initialize board strings (as before)
        board[0] = vec!["br", "bn", "bb", "bq", "bk", "bb", "bn", "br"];
        board[7] = vec!["wr", "wn", "wb", "wq", "wk", "wb", "wn", "wr"];

        // Initialize Kings
        pieces.push(Box::new(King::new(0, 4, 'b')));
        pieces.push(Box::new(King::new(7, 4, 'w')));

        // Initialize Queens
        pieces.push(Box::new(Queen::new(0, 3, 'b')));
        pieces.push(Box::new(Queen::new(7, 3, 'w')));

        // Initialize Bishops
        pieces.push(Box::new(Bishop::new(0, 2, 'b')));
        pieces.push(Box::new(Bishop::new(0, 5, 'b')));
        pieces.push(Box::new(Bishop::new(7, 2, 'w')));
        pieces.push(Box::new(Bishop::new(7, 5, 'w')));

        // Initialize Knights
        pieces.push(Box::new(Knight::new(0, 1, 'b')));
        pieces.push(Box::new(Knight::new(0, 6, 'b')));
        pieces.push(Box::new(Knight::new(7, 1, 'w')));
        pieces.push(Box::new(Knight::new(7, 6, 'w')));

        // Initialize Rooks
        pieces.push(Box::new(Rook::new(0, 0, 'b')));
        pieces.push(Box::new(Rook::new(0, 7, 'b')));
        pieces.push(Box::new(Rook::new(7, 0, 'w')));
        pieces.push(Box::new(Rook::new(7, 7, 'w')));

        // Initialize pawns and update board
        for i in 0..8 {
            let black_pawn = Pawn::new(1, i as i8, 'b');
            board[1][i] = "bp";
            pieces.push(Box::new(black_pawn));

            let white_pawn = Pawn::new(6, i as i8, 'w');
            board[6][i] = "wp";
            pieces.push(Box::new(white_pawn));
        }

        Self {
            board,
            turn: 'w',
            pieces,
            selected_figure_index: None,
            valid_moves: Vec::new(),
            history: Vec::new(),
            captured_by_white: Vec::new(),
            captured_by_black: Vec::new(),
            en_passant_target: None,
            is_check: false,
            is_checkmate: false,
            is_stalemate: false,
            promotion: None,
        }
    }

    pub fn is_square_attacked(&self, row: i8, col: i8, attacker_color: char) -> bool {
        for piece in &self.pieces {
            if piece.get_color() == attacker_color {
                let moves = piece.get_valid_moves(&self.board, self.en_passant_target);
                if moves.contains(&(row, col)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_in_check(&self, color: char) -> bool {
        let king_pos = self.pieces.iter()
            .find(|p| p.get_code().ends_with('k') && p.get_color() == color)
            .map(|p| p.get_pos());

        if let Some((r, c)) = king_pos {
            let attacker_color = if color == 'w' { 'b' } else { 'w' };
            return self.is_square_attacked(r, c, attacker_color);
        }
        false
    }

    pub fn get_legal_moves(&self, piece_index: usize) -> Vec<(i8, i8)> {
        let piece = &self.pieces[piece_index];
        let color = piece.get_color();
        let (old_row, old_col) = piece.get_pos();
        let pseudo_legal_moves = piece.get_valid_moves(&self.board, self.en_passant_target);
        let mut legal_moves = Vec::new();

        for (new_row, new_col) in pseudo_legal_moves {
            // Simulate move
            let mut temp_game_board = self.board.clone();
            let mut temp_pieces_codes = Vec::new();
            for p in &self.pieces {
                temp_pieces_codes.push((p.get_pos(), p.get_code(), p.get_color()));
            }

            // Perform move on the board clone and piece list clone
            let moving_piece_code = temp_game_board[old_row as usize][old_col as usize];
            temp_game_board[old_row as usize][old_col as usize] = "";

            // Handle capture in simulation
            let mut is_en_passant_capture = false;
            if let Some((ep_row, ep_col)) = self.en_passant_target {
                if new_row == ep_row && new_col == ep_col && moving_piece_code.ends_with('p') {
                    is_en_passant_capture = true;
                }
            }

            temp_game_board[new_row as usize][new_col as usize] = moving_piece_code;
            if is_en_passant_capture {
                let captured_pawn_row = if color == 'w' { new_row + 1 } else { new_row - 1 };
                temp_game_board[captured_pawn_row as usize][new_col as usize] = "";
            }

            // Check if king is in check in the simulated board
            if !self.simulated_is_in_check(&temp_game_board, color, (new_row, new_col), moving_piece_code, is_en_passant_capture) {
                legal_moves.push((new_row, new_col));
            }
        }

        // Add castling moves if the piece is a king
        if piece.get_code().ends_with('k') && !piece.has_moved() && !self.is_in_check(color) {
            let row: i8 = if color == 'w' { 7 } else { 0 };

            // Kingside castling
            if self.board[row as usize][5].is_empty() && self.board[row as usize][6].is_empty() {
                if self.pieces.iter().any(|p| {
                    let (pr, pc) = p.get_pos();
                    pr == row && pc == 7 && p.get_code().ends_with('r') && p.get_color() == color && !p.has_moved()
                }) {
                    // Check if squares are attacked
                    let opponent_color = if color == 'w' { 'b' } else { 'w' };
                    if !self.is_square_attacked(row, 5, opponent_color) &&
                        !self.is_square_attacked(row, 6, opponent_color) {
                        legal_moves.push((row, 6));
                    }
                }
            }

            // Queenside castling
            if self.board[row as usize][1].is_empty() && self.board[row as usize][2].is_empty() && self.board[row as usize][3].is_empty() {
                if self.pieces.iter().any(|p| {
                    let (pr, pc) = p.get_pos();
                    pr == row && pc == 0 && p.get_code().ends_with('r') && p.get_color() == color && !p.has_moved()
                }) {
                    // Check if squares are attacked (only 2 and 3 are traversed by king)
                    let opponent_color = if color == 'w' { 'b' } else { 'w' };
                    if !self.is_square_attacked(row, 2, opponent_color) &&
                        !self.is_square_attacked(row, 3, opponent_color) {
                        legal_moves.push((row, 2));
                    }
                }
            }
        }

        legal_moves
    }

    fn simulated_is_in_check(
        &self,
        board: &Vec<Vec<&'static str>>,
        color: char,
        new_pos: (i8, i8),
        moving_piece_code: &'static str,
        is_en_passant: bool
    ) -> bool {
        // Find king's position on the new board
        let mut king_pos = None;
        if moving_piece_code.ends_with('k') {
            king_pos = Some(new_pos);
        } else {
            for p in &self.pieces {
                if p.get_color() == color && p.get_code().ends_with('k') {
                    king_pos = Some(p.get_pos());
                    break;
                }
            }
        }

        let (kr, kc) = king_pos.expect("King should exist");
        let attacker_color = if color == 'w' { 'b' } else { 'w' };

        // We need to check if ANY piece of attacker_color can reach (kr, kc) on the NEW board.

        for piece in &self.pieces {
            if piece.get_color() == attacker_color {
                let (pr, pc) = piece.get_pos();

                // If this piece was captured, skip it
                if pr == new_pos.0 && pc == new_pos.1 {
                    continue;
                }
                if is_en_passant {
                    let captured_pawn_row = if color == 'w' { new_pos.0 + 1 } else { new_pos.0 - 1 };
                    if pr == captured_pawn_row && pc == new_pos.1 {
                        continue;
                    }
                }

                // Get valid moves for the attacker piece on the NEW board
                let moves = piece.get_valid_moves(board, None); // EP doesn't matter for checking king
                if moves.contains(&(kr, kc)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn clear_hints(&mut self) {
        for r in 0..8 {
            for c in 0..8 {
                if self.board[r][c] == "hint" {
                    self.board[r][c] = "";
                }
            }
        }
    }

    pub fn select_figure(&mut self, row: i8, col: i8) {
        // Clear previous hints from the board
        self.clear_hints();
        self.valid_moves.clear();
        self.selected_figure_index = None;

        // Find if a piece exists at (row, col)
        let mut found_index = None;
        for (index, piece) in self.pieces.iter().enumerate() {
            let (r, c) = piece.get_pos();
            if r == row && c == col {
                // Only allow selecting pieces of the current turn's color
                if piece.get_color() == self.turn {
                    found_index = Some(index);
                }
                break;
            }
        }

        if let Some(index) = found_index {
            self.selected_figure_index = Some(index as i8);
            self.valid_moves = self.get_legal_moves(index);

            // Show hints on board for empty squares
            for &(mr, mc) in &self.valid_moves {
                if self.board[mr as usize][mc as usize].is_empty() {
                    self.board[mr as usize][mc as usize] = "hint";
                }
            }
        }
    }

    pub fn move_selected_piece(&mut self, new_row: i8, new_col: i8) {
        if let Some(index_i8) = self.selected_figure_index {
            let index = index_i8 as usize;
            if self.valid_moves.contains(&(new_row, new_col)) {
                // Clear hints before moving
                self.clear_hints();

                // Handle capture
                let (old_row, old_col) = self.pieces[index].get_pos();
                let target_piece = self.board[new_row as usize][new_col as usize];

                // En passant capture detection
                let mut is_en_passant_capture = false;
                if let Some((ep_row, ep_col)) = self.en_passant_target {
                    if new_row == ep_row && new_col == ep_col {
                        if self.pieces[index].get_code().ends_with('p') {
                            is_en_passant_capture = true;
                        }
                    }
                }

                if !target_piece.is_empty() && target_piece != "hint" {
                    // Record captured piece
                    if self.turn == 'w' {
                        self.captured_by_white.push(target_piece);
                    } else {
                        self.captured_by_black.push(target_piece);
                    }

                    // Find and remove captured piece
                    self.pieces.retain(|p| {
                        let (r, c) = p.get_pos();
                        r != new_row || c != new_col
                    });
                } else if is_en_passant_capture {
                    let captured_pawn_row = if self.turn == 'w' { new_row + 1 } else { new_row - 1 };
                    let captured_pawn_col = new_col;
                    let captured_piece_code = self.board[captured_pawn_row as usize][captured_pawn_col as usize];

                    if self.turn == 'w' {
                        self.captured_by_white.push(captured_piece_code);
                    } else {
                        self.captured_by_black.push(captured_piece_code);
                    }

                    // Remove the captured pawn from pieces list
                    self.pieces.retain(|p| {
                        let (r, c) = p.get_pos();
                        r != captured_pawn_row || c != captured_pawn_col
                    });

                    // Clear the captured pawn's position on the board
                    self.board[captured_pawn_row as usize][captured_pawn_col as usize] = "";
                }

                // Find the moving piece again because index might have changed after retain
                if let Some(piece_index) = self.pieces.iter().position(|p| {
                    let (r, c) = p.get_pos();
                    r == old_row && c == old_col
                }) {
                    let mut next_en_passant_target = None;
                    let current_piece_code = self.pieces[piece_index].get_code();
                    if current_piece_code.ends_with('p') {
                        if (new_row - old_row).abs() == 2 {
                            next_en_passant_target = Some(((old_row + new_row) / 2, old_col));
                        }
                    }
                    self.en_passant_target = next_en_passant_target;

                    let move_str = format!("{}: {}{} -> {}{}",
                                           current_piece_code,
                                           (old_col as u8 + b'a') as char, 8 - old_row,
                                           (new_col as u8 + b'a') as char, 8 - new_row
                    );
                    self.history.push(move_str);

                    // Handle castling rook move
                    if current_piece_code.ends_with('k') && (new_col - old_col).abs() == 2 {
                        let rook_old_col = if new_col == 6 { 7 } else { 0 };
                        let rook_new_col = if new_col == 6 { 5 } else { 3 };
                        if let Some(rook_index) = self.pieces.iter().position(|p| {
                            p.get_pos() == (new_row, rook_old_col) && p.get_code().ends_with('r')
                        }) {
                            self.pieces[rook_index].move_piece(new_row, rook_new_col, &mut self.board);
                        }
                    }

                    self.pieces[piece_index].move_piece(new_row, new_col, &mut self.board);

                    // Toggle turn if not promoting
                    if (new_row == 0 && current_piece_code == "wp") || (new_row == 7 && current_piece_code == "bp") {
                        self.promotion = Some(piece_index);
                    } else {
                        self.turn = if self.turn == 'w' { 'b' } else { 'w' };
                        // Update game status (check, checkmate, stalemate)
                        self.update_game_status();
                    }
                }

                self.selected_figure_index = None;
                self.valid_moves.clear();
            }
        }
    }

    pub fn promote_pawn(&mut self, captured_index: usize) {
        if let Some(pawn_index) = self.promotion {
            let (row, col) = self.pieces[pawn_index].get_pos();
            let color = self.pieces[pawn_index].get_color();

            let piece_code = if color == 'w' {
                self.captured_by_black.remove(captured_index)
            } else {
                self.captured_by_white.remove(captured_index)
            };

            let new_piece: Box<dyn Piece> = match &piece_code[1..] {
                "q" => Box::new(Queen::new(row, col, color)),
                "r" => Box::new(Rook::new(row, col, color)),
                "b" => Box::new(Bishop::new(row, col, color)),
                "n" => Box::new(Knight::new(row, col, color)),
                _ => Box::new(Queen::new(row, col, color)), // Default to queen if something's wrong
            };

            self.pieces[pawn_index] = new_piece;
            self.board[row as usize][col as usize] = piece_code;

            self.promotion = None;
            self.turn = if self.turn == 'w' { 'b' } else { 'w' };
            self.update_game_status();
        }
    }

    fn update_game_status(&mut self) {
        self.is_check = self.is_in_check(self.turn);

        let mut has_legal_moves = false;
        for i in 0..self.pieces.len() {
            if self.pieces[i].get_color() == self.turn {
                let moves = self.get_legal_moves(i);
                if !moves.is_empty() {
                    has_legal_moves = true;
                    break;
                }
            }
        }

        if !has_legal_moves {
            if self.is_check {
                self.is_checkmate = true;
            } else {
                self.is_stalemate = true;
            }
        } else {
            self.is_checkmate = false;
            self.is_stalemate = false;
        }
    }

    pub fn reset(&mut self) {
        let new_game = Self::new();
        self.board = new_game.board;
        self.turn = new_game.turn;
        self.pieces = new_game.pieces;
        self.selected_figure_index = new_game.selected_figure_index;
        self.valid_moves = new_game.valid_moves;
        self.history = new_game.history;
        self.captured_by_white = new_game.captured_by_white;
        self.captured_by_black = new_game.captured_by_black;
        self.en_passant_target = new_game.en_passant_target;
        self.is_check = new_game.is_check;
        self.is_checkmate = new_game.is_checkmate;
        self.is_stalemate = new_game.is_stalemate;
        self.promotion = new_game.promotion;
    }

    pub fn move_piece(&mut self, piece_index: usize, new_row: i8, new_col: i8) {
        if let Some(piece) = self.pieces.get_mut(piece_index) {
            piece.move_piece(new_row, new_col, &mut self.board);
        }
    }
}