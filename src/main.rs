mod render_board;

use std::error::Error;
use render_board::render_app;

fn main() -> Result<(), Box<dyn Error>> {
    let chess_board = create_chess_board();
    render_app::run_game(&chess_board)
}

fn create_chess_board() -> Vec<Vec<&'static str>> {
    let mut chess_board = vec![vec![""; 8]; 8];

    chess_board[0] = vec!["br", "bh", "bb", "bq", "bk", "bb", "bh", "br"];
    chess_board[1] = vec!["bp"; 8];
    chess_board[6] = vec!["wp"; 8];
    chess_board[7] = vec!["wr", "wh", "wb", "wq", "wk", "wb", "wh", "wr"];

    // es - empty space
    // w - white
    // b - black
    // r - rook
    // b - bishop
    // h - hours
    // q - queen
    // k - king
    // ["wp", "wp", "wp", "wp", "wp", "wp", "wp", "wp"]
    // ["wr", "wb", "wh", "wq", "wk", "wh", "wb", "wr"]
    // ["es", "es", "es", "es", "es", "es", "es", "es"]
    // ["es", "es", "es", "es", "es", "es", "es", "es"]
    // ["es", "es", "es", "es", "es", "es", "es", "es"]
    // ["es", "es", "es", "es", "es", "es", "es", "es"]
    // ["es", "es", "es", "es", "es", "es", "es", "es"]
    // ["bp", "bp", "bp", "bp", "bp", "bp", "bp", "bp"]
    // ["br", "bb", "bh", "bq", "bk", "bh", "bb", "br"]

    chess_board
}