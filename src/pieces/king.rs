use super::piece::Piece;
use super::moves_controller::single_step_controller;
pub struct King {
    pub row: i8,
    pub col: i8,
    pub color: char,
}

impl Piece for King {
    fn get_pos(&self) -> (i8, i8) { (self.row, self.col) }

    fn move_piece(&mut self, new_row: i8, new_col: i8, board: &mut Vec<Vec<&'static str>>) {
        self.move_king(new_row, new_col, board);
    }

    fn get_valid_moves(&self, board: &Vec<Vec<&'static str>>, _en_passant_target: Option<(i8, i8)>) -> Vec<(i8, i8)> {
        let directions = [
            (-1, 0), (1, 0), (0, -1), (0, 1),
            (-1, -1), (-1, 1), (1, -1), (1, 1)
        ];

        single_step_controller(self.row, self.col, self.color, &directions, board)
    }

    fn get_color(&self) -> char {
        self.color
    }

    fn get_code(&self) -> &'static str {
        if self.color == 'w' { "wk" } else { "bk" }
    }
}

impl King {
    pub fn new(row: i8, col: i8, color: char) -> Self {
        Self { row, col, color }
    }

    pub fn move_king(&mut self, new_row: i8, new_col: i8, board: &mut Vec<Vec<&'static str>>) {
        let piece_code = self.get_code();

        // Clear old position
        board[self.row as usize][self.col as usize] = "";

        // Update to new position
        self.row = new_row;
        self.col = new_col;
        board[self.row as usize][self.col as usize] = piece_code;
    }
}