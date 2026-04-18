mod render_board;
mod pieces;

use std::error::Error;
use render_board::render_app;
use pieces::pawn::Pawn;
use pieces::king::King;
use pieces::queen::Queen;
use pieces::bishop::Bishop;
use pieces::knight::Knight;
use pieces::rook::Rook;
use pieces::piece::Piece;

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
        }
    }

    pub fn select_figure(&mut self, row: i8, col: i8) {
        // Clear previous hints from the board
        for r in 0..8 {
            for c in 0..8 {
                if self.board[r][c] == "hint" {
                    self.board[r][c] = "";
                }
            }
        }
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
            self.valid_moves = self.pieces[index].get_valid_moves(&self.board, self.en_passant_target);
            
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
                for r in 0..8 {
                    for c in 0..8 {
                        if self.board[r][c] == "hint" {
                            self.board[r][c] = "";
                        }
                    }
                }

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
                    let piece_code = self.pieces[piece_index].get_code();
                    
                    // Update en_passant_target for next turn
                    let mut next_en_passant_target = None;
                    if piece_code.ends_with('p') {
                        if (new_row - old_row).abs() == 2 {
                            next_en_passant_target = Some(((old_row + new_row) / 2, old_col));
                        }
                    }
                    self.en_passant_target = next_en_passant_target;

                    let move_str = format!("{}: {}{} -> {}{}", 
                        piece_code,
                        (old_col as u8 + b'a') as char, 8 - old_row,
                        (new_col as u8 + b'a') as char, 8 - new_row
                    );
                    self.history.push(move_str);

                    self.pieces[piece_index].move_piece(new_row, new_col, &mut self.board);
                }
                
                self.selected_figure_index = None;
                self.valid_moves.clear();
                
                // Toggle turn
                self.turn = if self.turn == 'w' { 'b' } else { 'w' };
            }
        }
    }

    pub fn move_piece(&mut self, piece_index: i8, new_row: i8, new_col: i8) {
        if let Some(piece) = self.pieces.get_mut(piece_index as usize) {
            piece.move_piece(new_row, new_col, &mut self.board);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    render_app::run_game(&mut game)
}