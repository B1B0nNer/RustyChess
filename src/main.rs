mod game;
mod render_board;
mod pieces;

use std::error::Error;
use game::game::Game;

#[cfg(test)]
mod test;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    render_board::render_app::run_game(&mut game)
}