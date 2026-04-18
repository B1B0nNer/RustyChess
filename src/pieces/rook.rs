use super::piece::Piece;
use super::moves_controller::moves_controller;
pub struct Rook {
    pub row: i8,
    pub col: i8,
    pub color: char,
}

impl Piece for Rook {
    fn get_pos(&self) -> (i8, i8) { (self.row, self.col) }

    fn move_piece(&mut self, new_row: i8, new_col: i8, board: &mut Vec<Vec<&'static str>>) {
        self.move_rook(new_row, new_col, board);
    }

    fn get_valid_moves(&self, board: &Vec<Vec<&'static str>>) -> Vec<(i8, i8)> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        moves_controller(self.row, self.col, self.color, &directions, board)
    }

    fn get_color(&self) -> char {
        self.color
    }

    fn get_code(&self) -> &'static str {
        if self.color == 'w' { "wr" } else { "br" }
    }
}

impl Rook {
    pub fn new(row: i8, col: i8, color: char) -> Self {
        Self { row, col, color }
    }

    pub fn move_rook(&mut self, new_row: i8, new_col: i8, board: &mut Vec<Vec<&'static str>>) {
        let piece_code = self.get_code();

        // Clear old position
        board[self.row as usize][self.col as usize] = "";

        // Update to new position
        self.row = new_row;
        self.col = new_col;
        board[self.row as usize][self.col as usize] = piece_code;
    }
}