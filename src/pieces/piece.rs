pub trait Piece: Send + Sync {
    fn get_pos(&self) -> (usize, usize);
    fn move_piece(&mut self, new_row: usize, new_col: usize, board: &mut Vec<Vec<&'static str>>);
    fn get_valid_moves(&self, board: &Vec<Vec<&'static str>>) -> Vec<(usize, usize)>;
    fn get_color(&self) -> char;
    fn get_code(&self) -> &'static str;
}