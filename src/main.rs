use std::process;
//use crossterm::terminal;

mod tetris;
mod draw;

use tetris::Game;

fn main() {
    let mut game: Game<10, 20> = Game::new();

    //terminal::enable_raw_mode().unwrap();
    
    let r = game.run().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    //terminal::disable_raw_mode().unwrap();

    process::exit(r as i32);
}
