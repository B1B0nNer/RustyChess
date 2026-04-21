use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::bishop::Bishop;
use crate::pieces::knight::Knight;
use crate::pieces::piece::Piece;
use crate::Game;

pub fn promote_pawn(game: &mut Game, captured_index: usize) {
    if let Some(pawn_index) = game.promotion {
        let (row, col) = game.pieces[pawn_index].get_pos();
        let color = game.pieces[pawn_index].get_color();

        let piece_code = if color == 'w' {
            game.captured_by_black.remove(captured_index)
        } else {
            game.captured_by_white.remove(captured_index)
        };

        let new_piece: Box<dyn Piece> = match &piece_code[1..] {
            "q" => Box::new(Queen::new(row, col, color)),
            "r" => Box::new(Rook::new(row, col, color)),
            "b" => Box::new(Bishop::new(row, col, color)),
            "n" => Box::new(Knight::new(row, col, color)),
            _ => Box::new(Queen::new(row, col, color)), // Default to queen if something's wrong
        };

        game.pieces[pawn_index] = new_piece;
        game.board[row as usize][col as usize] = piece_code;

        game.promotion = None;
        game.turn = if game.turn == 'w' { 'b' } else { 'w' };
        game.update_game_status();
    }
}
