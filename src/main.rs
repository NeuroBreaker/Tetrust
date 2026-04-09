use std::process;

mod tetris;
mod draw;

fn main() {
    let game = tetris::Game::new();
    let result = game.run();
}
