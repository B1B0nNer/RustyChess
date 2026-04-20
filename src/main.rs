mod game;
mod render_board;
mod pieces;

use std::error::Error;
use game::Game;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    render_board::render_app::run_game(&mut game)
}