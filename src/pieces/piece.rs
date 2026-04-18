pub trait Piece: Send + Sync {
    fn get_pos(&self) -> (i8, i8);
    fn move_piece(&mut self, new_row: i8, new_col: i8, board: &mut Vec<Vec<&'static str>>);
    fn get_valid_moves(&self, board: &Vec<Vec<&'static str>>, en_passant_target: Option<(i8, i8)>) -> Vec<(i8, i8)>;
    fn get_color(&self) -> char;
    fn get_code(&self) -> &'static str;
}