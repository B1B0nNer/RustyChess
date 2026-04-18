mod render_board;
mod pieces;

use std::error::Error;
use render_board::render_app;
use pieces::pawn::Pawn;
use pieces::piece::Piece;

pub struct Game {
    pub board: Vec<Vec<&'static str>>,
    pub turn: char,
    pub pieces: Vec<Box<dyn Piece>>,
    pub selected_figure_index: Option<usize>,
    pub valid_moves: Vec<(usize, usize)>,
    pub history: Vec<String>,
    pub captured_by_white: Vec<&'static str>,
    pub captured_by_black: Vec<&'static str>,
}

impl Game {
    pub fn new() -> Self {
        let mut board = vec![vec![""; 8]; 8];
        let mut pieces: Vec<Box<dyn Piece>> = Vec::new();

        // Initialize board strings (as before)
        board[0] = vec!["br", "bh", "bb", "bq", "bk", "bb", "bh", "br"];
        board[7] = vec!["wr", "wh", "wb", "wq", "wk", "wb", "wh", "wr"];

        // Initialize pawns and update board
        for i in 0..8 {
            let black_pawn = Pawn::new(1, i, 'b');
            board[1][i] = "bp";
            pieces.push(Box::new(black_pawn));

            let white_pawn = Pawn::new(6, i, 'w');
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
        }
    }

    pub fn select_figure(&mut self, row: usize, col: usize) {
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
            self.selected_figure_index = Some(index);
            self.valid_moves = self.pieces[index].get_valid_moves(&self.board);
            
            // Show hints on board for empty squares
            for &(mr, mc) in &self.valid_moves {
                if self.board[mr][mc].is_empty() {
                    self.board[mr][mc] = "hint";
                }
            }
        }
    }

    pub fn move_selected_piece(&mut self, new_row: usize, new_col: usize) {
        if let Some(index) = self.selected_figure_index {
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
                let target_piece = self.board[new_row][new_col];
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
                }
                
                // Find the moving piece again because index might have changed after retain
                if let Some(piece_index) = self.pieces.iter().position(|p| {
                    let (r, c) = p.get_pos();
                    r == old_row && c == old_col
                }) {
                    let piece_code = self.pieces[piece_index].get_code();
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

    pub fn move_piece(&mut self, piece_index: usize, new_row: usize, new_col: usize) {
        if let Some(piece) = self.pieces.get_mut(piece_index) {
            piece.move_piece(new_row, new_col, &mut self.board);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    render_app::run_game(&mut game)
}