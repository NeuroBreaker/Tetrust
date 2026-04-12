use std::process;
use crossterm::terminal;

mod tetris;
mod draw;

use tetris::Game;

fn main() {
    let mut game: Game<10, 20> = Game::new();

    terminal::enable_raw_mode().expect("Couldn't turn on raw mode");
    
    let r = game.run().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    terminal::disable_raw_mode().expect("Couldn't turn on raw mode");

    process::exit(r);
}
