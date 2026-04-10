use std::process;

mod tetris;
mod draw;

use tetris::Game;

fn main() {
    let game: Game<10, 20> = Game::new();
    let result = game.run();
}
