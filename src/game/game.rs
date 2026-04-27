use strum::{Display, EnumIter};
use crate::pieces::pawn::Pawn;
use crate::pieces::king::King;
use crate::pieces::queen::Queen;
use crate::pieces::bishop::Bishop;
use crate::pieces::knight::Knight;
use crate::pieces::rook::Rook;
use crate::pieces::piece::Piece;

pub enum GameMode {
    Normal,
    Fischer,
}

#[derive(Debug, Display, EnumIter, PartialEq)]
pub enum TimeMode {
    #[strum(serialize = "No Time")]
    Unlimited,
    #[strum(serialize = "1 Min")]
    OneMinute,
    #[strum(serialize = "3 Min")]
    ThreeMinutes,
    #[strum(serialize = "5 Min")]
    FiveMinutes,
    #[strum(serialize = "10 Min")]
    TenMinutes,
    #[strum(serialize = "15 Min")]
    FifteenMinutes,
    #[strum(serialize = "30 Min")]
    ThirtyMinutes,
    #[strum(serialize = "45 Min")]
    FortyFiveMinutes,
    #[strum(serialize = "1 Hour")]
    OneHour,
}

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
    pub game_mode: Option<GameMode>,
    pub time_mode: Option<TimeMode>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: vec![vec![""; 8]; 8],
            turn: 'w',
            pieces: Vec::new(),
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
            game_mode: None,
            time_mode: None,
        }
    }

    pub fn init_normal(&mut self) {
        self.game_mode = Some(GameMode::Normal);
        self.board = vec![vec![""; 8]; 8];
        self.pieces = Vec::new();

        // Initialize board strings
        self.board[0] = vec!["br", "bn", "bb", "bq", "bk", "bb", "bn", "br"];
        self.board[7] = vec!["wr", "wn", "wb", "wq", "wk", "wb", "wn", "wr"];

        // Initialize Kings
        self.pieces.push(Box::new(King::new(0, 4, 'b')));
        self.pieces.push(Box::new(King::new(7, 4, 'w')));

        // Initialize Queens
        self.pieces.push(Box::new(Queen::new(0, 3, 'b')));
        self.pieces.push(Box::new(Queen::new(7, 3, 'w')));

        // Initialize Bishops
        self.pieces.push(Box::new(Bishop::new(0, 2, 'b')));
        self.pieces.push(Box::new(Bishop::new(0, 5, 'b')));
        self.pieces.push(Box::new(Bishop::new(7, 2, 'w')));
        self.pieces.push(Box::new(Bishop::new(7, 5, 'w')));

        // Initialize Knights
        self.pieces.push(Box::new(Knight::new(0, 1, 'b')));
        self.pieces.push(Box::new(Knight::new(0, 6, 'b')));
        self.pieces.push(Box::new(Knight::new(7, 1, 'w')));
        self.pieces.push(Box::new(Knight::new(7, 6, 'w')));

        // Initialize Rooks
        self.pieces.push(Box::new(Rook::new(0, 0, 'b')));
        self.pieces.push(Box::new(Rook::new(0, 7, 'b')));
        self.pieces.push(Box::new(Rook::new(7, 0, 'w')));
        self.pieces.push(Box::new(Rook::new(7, 7, 'w')));

        // Initialize pawns and update board
        for i in 0..8 {
            let black_pawn = Pawn::new(1, i as i8, 'b');
            self.board[1][i] = "bp";
            self.pieces.push(Box::new(black_pawn));

            let white_pawn = Pawn::new(6, i as i8, 'w');
            self.board[6][i] = "wp";
            self.pieces.push(Box::new(white_pawn));
        }
    }

    pub fn init_fischer(&mut self) {
        self.game_mode = Some(GameMode::Fischer);
        self.board = vec![vec![""; 8]; 8];
        self.pieces = Vec::new();

        use rand::seq::SliceRandom;
        use rand::prelude::IteratorRandom;
        let mut rng = rand::thread_rng();

        // 1. Place Bishops on opposite colors
        let mut positions: Vec<usize> = (0..8).collect();
        let light_bishop_pos = *[1, 3, 5, 7].choose(&mut rng).unwrap();
        let dark_bishop_pos = *[0, 2, 4, 6].choose(&mut rng).unwrap();
        positions.retain(|&x| x != light_bishop_pos && x != dark_bishop_pos);

        // 2. Place Queen
        let queen_idx = (0..positions.len()).choose(&mut rng).unwrap();
        let queen_pos = positions.remove(queen_idx);

        // 3. Place Knights
        let n1_idx = (0..positions.len()).choose(&mut rng).unwrap();
        let n1_pos = positions.remove(n1_idx);
        let n2_idx = (0..positions.len()).choose(&mut rng).unwrap();
        let n2_pos = positions.remove(n2_idx);

        // 4. Place Rooks and King (King must be between Rooks)
        positions.sort();
        let r1_pos = positions[0];
        let king_pos = positions[1];
        let r2_pos = positions[2];

        let mut row0 = vec![""; 8];
        row0[light_bishop_pos] = "bb";
        row0[dark_bishop_pos] = "bb";
        row0[queen_pos] = "bq";
        row0[n1_pos] = "bn";
        row0[n2_pos] = "bn";
        row0[r1_pos] = "br";
        row0[king_pos] = "bk";
        row0[r2_pos] = "br";
        self.board[0] = row0;

        let mut row7 = vec![""; 8];
        row7[light_bishop_pos] = "wb";
        row7[dark_bishop_pos] = "wb";
        row7[queen_pos] = "wq";
        row7[n1_pos] = "wn";
        row7[n2_pos] = "wn";
        row7[r1_pos] = "wr";
        row7[king_pos] = "wk";
        row7[r2_pos] = "wr";
        self.board[7] = row7;

        // Initialize pieces
        for (i, &code) in self.board[0].iter().enumerate() {
            match code {
                "bk" => self.pieces.push(Box::new(King::new(0, i as i8, 'b'))),
                "bq" => self.pieces.push(Box::new(Queen::new(0, i as i8, 'b'))),
                "bb" => self.pieces.push(Box::new(Bishop::new(0, i as i8, 'b'))),
                "bn" => self.pieces.push(Box::new(Knight::new(0, i as i8, 'b'))),
                "br" => self.pieces.push(Box::new(Rook::new(0, i as i8, 'b'))),
                _ => {}
            }
        }
        for (i, &code) in self.board[7].iter().enumerate() {
            match code {
                "wk" => self.pieces.push(Box::new(King::new(7, i as i8, 'w'))),
                "wq" => self.pieces.push(Box::new(Queen::new(7, i as i8, 'w'))),
                "wb" => self.pieces.push(Box::new(Bishop::new(7, i as i8, 'w'))),
                "wn" => self.pieces.push(Box::new(Knight::new(7, i as i8, 'w'))),
                "wr" => self.pieces.push(Box::new(Rook::new(7, i as i8, 'w'))),
                _ => {}
            }
        }

        // Initialize pawns
        for i in 0..8 {
            self.board[1][i] = "bp";
            self.pieces.push(Box::new(Pawn::new(1, i as i8, 'b')));
            self.board[6][i] = "wp";
            self.pieces.push(Box::new(Pawn::new(6, i as i8, 'w')));
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


    pub fn update_game_status(&mut self) {
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
        self.game_mode = new_game.game_mode;
        self.time_mode = new_game.time_mode;
    }
}