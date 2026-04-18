use super::piece::Piece;
pub struct Pawn {
    pub row: usize,
    pub col: usize,
    pub color: char, // 'w' for white, 'b' for black
    pub first_move: bool,
    pub en_passant: bool,
    pub promotion: bool,
}

impl Piece for Pawn {
    fn get_pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn move_piece(&mut self, new_row: usize, new_col: usize, board: &mut Vec<Vec<&'static str>>) {
        self.move_pawn(new_row, new_col, board);
    }

    fn get_valid_moves(&self, board: &Vec<Vec<&'static str>>) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let direction: i8 = if self.color == 'w' { -1 } else { 1 };
        let start_row = self.row as i8;
        let start_col = self.col as i8;

        // Forward move
        let next_row = start_row + direction;
        if next_row >= 0 && next_row < 8 {
            if board[next_row as usize][self.col].is_empty() {
                moves.push((next_row as usize, self.col));

                // Double move on first move
                if self.first_move {
                    let double_row = start_row + 2 * direction;
                    if double_row >= 0 && double_row < 8 {
                        if board[double_row as usize][self.col].is_empty() {
                            moves.push((double_row as usize, self.col));
                        }
                    }
                }
            }
        }

        // Captures
        for &offset_col in &[-1, 1] {
            let target_col = start_col + offset_col;
            if target_col >= 0 && target_col < 8 {
                let target_row = start_row + direction;
                if target_row >= 0 && target_row < 8 {
                    let piece = board[target_row as usize][target_col as usize];
                    if !piece.is_empty() && piece.chars().next().unwrap() != self.color {
                        moves.push((target_row as usize, target_col as usize));
                    }
                }
            }
        }

        moves
    }

    fn get_color(&self) -> char {
        self.color
    }

    fn get_code(&self) -> &'static str {
        if self.color == 'w' { "wp" } else { "bp" }
    }
}

impl Pawn {
    pub fn new(row: usize, col: usize, color: char) -> Self {
        Self {
            row,
            col,
            color,
            first_move: true,
            en_passant: false,
            promotion: false,
        }
    }

    pub fn move_pawn(&mut self, new_row: usize, new_col: usize, board: &mut Vec<Vec<&'static str>>) {
        // Simple logic for demonstration: update board and internal state
        let piece_code = if self.color == 'w' { "wp" } else { "bp" };
        
        // Clear old position
        board[self.row][self.col] = "";
        
        // Update to new position
        self.row = new_row;
        self.col = new_col;
        board[self.row][self.col] = piece_code;
        
        if self.first_move {
            self.first_move = false;
        }
    }
}