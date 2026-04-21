use super::super::Game;

pub fn new_with_board() -> Game {
    let mut game = Game::new();
    game.init_normal();
    game
}

#[test]
fn test_game_initialization() {
    let game = new_with_board();
    assert_eq!(game.turn, 'w');
    assert_eq!(game.board[0][0], "br");
    assert_eq!(game.board[7][4], "wk");
    assert_eq!(game.pieces.len(), 32);
}

#[test]
fn test_pawn_initial_moves() {
    let game = new_with_board();
    let pawn_index = game.pieces.iter().position(|p| p.get_pos() == (6, 4)).unwrap();
    let moves = game.get_legal_moves(pawn_index);
    assert!(moves.contains(&(5, 4)));
    assert!(moves.contains(&(4, 4)));
    assert_eq!(moves.len(), 2);
}

#[test]
fn test_knight_initial_moves() {
    let game = new_with_board();
    let knight_index = game.pieces.iter().position(|p| p.get_pos() == (7, 1)).unwrap();
    let moves = game.get_legal_moves(knight_index);
    assert!(moves.contains(&(5, 0)));
    assert!(moves.contains(&(5, 2)));
    assert_eq!(moves.len(), 2);
}

#[test]
fn test_simple_move() {
    let mut game = new_with_board();
    game.select_figure(6, 4);
    game.move_selected_piece(4, 4);
    assert_eq!(game.board[6][4], "");
    assert_eq!(game.board[4][4], "wp");
    assert_eq!(game.turn, 'b');
}

#[test]
fn test_check_detection() {
    let mut game = new_with_board();
    game.select_figure(6, 4); game.move_selected_piece(4, 4); // 1. e4
    game.select_figure(1, 4); game.move_selected_piece(3, 4); // 1... e5
    game.select_figure(7, 5); game.move_selected_piece(4, 2); // 2. Bc4
    game.select_figure(0, 1); game.move_selected_piece(2, 2); // 2... Nc6
    game.select_figure(7, 3); game.move_selected_piece(3, 7); // 3. Qh5
    game.select_figure(0, 6); game.move_selected_piece(2, 5); // 3... Nf6
    game.select_figure(3, 7); game.move_selected_piece(1, 5); // 4. Qxf7+
    
    assert!(game.is_in_check('b'));
    assert!(game.is_check);
}

#[test]
fn test_en_passant_target() {
    let mut game = new_with_board();
    game.select_figure(6, 4);
    game.move_selected_piece(4, 4);
    assert_eq!(game.en_passant_target, Some((5, 4)));
    
    game.select_figure(1, 0);
    game.move_selected_piece(2, 0);
    assert_eq!(game.en_passant_target, None);
}

#[test]
fn test_en_passant_capture() {
    let mut game = new_with_board();
    game.select_figure(6, 4); game.move_selected_piece(4, 4);
    game.select_figure(1, 0); game.move_selected_piece(2, 0);
    game.select_figure(4, 4); game.move_selected_piece(3, 4);
    game.select_figure(1, 5); game.move_selected_piece(3, 5);
    
    assert_eq!(game.en_passant_target, Some((2, 5)));
    game.select_figure(3, 4);
    assert!(game.valid_moves.contains(&(2, 5)));
    game.move_selected_piece(2, 5);
    
    assert_eq!(game.board[3][5], "");
    assert_eq!(game.board[2][5], "wp");
    assert!(game.captured_by_white.contains(&"bp"));
}

#[test]
fn test_promotion() {
    let mut game = new_with_board();
    game.captured_by_black.push("bq");

    game.select_figure(6, 4); game.move_selected_piece(4, 4);
    game.select_figure(1, 3); game.move_selected_piece(3, 3);
    game.select_figure(4, 4); game.move_selected_piece(3, 3);
    game.select_figure(1, 2); game.move_selected_piece(3, 2);
    game.select_figure(3, 3); game.move_selected_piece(2, 2);
    game.select_figure(1, 4); game.move_selected_piece(3, 4);
    game.select_figure(2, 2); game.move_selected_piece(1, 1);
    game.select_figure(1, 0); game.move_selected_piece(2, 0);

    game.select_figure(1, 1);
    assert!(game.valid_moves.contains(&(0, 0)));
    game.move_selected_piece(0, 0);
    
    assert!(game.promotion.is_some());
    let cap_idx = game.captured_by_black.iter().position(|&p| p == "bq").unwrap();
    crate::game::promotion::promote_pawn(&mut game, cap_idx);
    
    assert_eq!(game.board[0][0], "bq"); 
    assert_eq!(game.turn, 'b');
}

#[test]
fn test_castling() {
    let mut game = new_with_board();

    game.select_figure(6, 4); game.move_selected_piece(4, 4);
    game.select_figure(1, 4); game.move_selected_piece(3, 4);

    game.select_figure(7, 6); game.move_selected_piece(5, 5);
    game.select_figure(0, 1); game.move_selected_piece(2, 2);

    game.select_figure(7, 5); game.move_selected_piece(4, 2);
    game.select_figure(0, 5); game.move_selected_piece(3, 2);

    game.select_figure(7, 4);
    assert!(game.valid_moves.contains(&(7, 6)));
    game.move_selected_piece(7, 6);
    
    assert_eq!(game.board[7][6], "wk");
    assert_eq!(game.board[7][5], "wr");
    assert_eq!(game.board[7][4], "");
    assert_eq!(game.board[7][7], "");
}

#[test]
fn test_checkmate() {
    let mut game = new_with_board();

    game.select_figure(6, 5); game.move_selected_piece(5, 5);
    game.select_figure(1, 4); game.move_selected_piece(3, 4);
    game.select_figure(6, 6); game.move_selected_piece(4, 6);
    game.select_figure(0, 3); game.move_selected_piece(4, 7);
    
    assert!(game.is_checkmate);
}
