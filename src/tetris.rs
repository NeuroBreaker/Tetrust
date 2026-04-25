use crossterm::event::{self, Event, KeyCode};
use rand::random_range;
use std::{
    process,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::draw::Draw;

#[derive(Clone, Copy)]
struct Piece {
    shape: [[u8; 4]; 4],
    size: usize,
}

impl Piece {
    pub fn new(piece: &[&[u8]], size: usize) -> Self {
        let mut shape = [[0u8; 4]; 4];
        for row in 0..piece.len() {
            for col in 0..piece[0].len() {
                shape[row][col] = piece[row][col];
            }
        }

        Self { shape, size }
    }
}

pub struct Game<const W: usize, const H: usize> {
    board: [[u8; W]; H],
    width: usize,
    height: usize,
    pieces: [Piece; 7],
    current_piece: Option<Piece>,
    current_color: u8,
    current_x: i32,
    current_y: i32,
    score: u128,
    game_over: bool,
}

impl<const W: usize, const H: usize> Game<W, H> {
    pub fn new() -> Self {
        let pieces: [Piece; 7] = [
            Piece::new(&[&[1, 1, 1, 1]], 4),
            Piece::new(&[&[1, 1], &[1, 1]], 2),
            Piece::new(&[&[0, 1, 0], &[1, 1, 1]], 3),
            Piece::new(&[&[0, 1, 1], &[1, 1, 0]], 3),
            Piece::new(&[&[1, 1, 0], &[0, 1, 1]], 3),
            Piece::new(&[&[1, 0, 0], &[1, 1, 1]], 3),
            Piece::new(&[&[0, 0, 1], &[1, 1, 1]], 3),
        ];

        Self {
            board: [[0u8; W]; H],
            width: W,
            height: H,
            pieces,
            current_piece: None,
            current_color: 0,
            current_x: 0,
            current_y: 0,
            score: 0,
            game_over: false,
        }
    }

    fn check_collision(&self, x: i32, y: i32, piece: &[[u8; 4]; 4]) -> bool {
        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, &cell_value) in row.iter().enumerate() {
                if cell_value != 0 {
                    let board_x = x + col_idx as i32;
                    let board_y = y + row_idx as i32;

                    if board_x < 0 || board_x >= self.width as i32 || board_y >= self.height as i32
                    {
                        return true;
                    }

                    if board_y >= 0 && self.board[board_y as usize][board_x as usize] != 0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn new_game(&mut self) {
        self.game_over = false;
        self.board = [[0u8; W]; H];
        self.score = 0;
        self.spawn_piece();
    }

    fn spawn_piece(&mut self) {
        let piece_index = random_range(0..self.pieces.len());
        let piece = self.pieces[piece_index];
        self.current_color = piece_index as u8 + 1;
        self.current_x = self.width as i32 / 2 - 2;
        self.current_y = -1;

        if self.check_collision(self.current_x, self.current_y, &piece.shape) {
            self.game_over = true;
        }
        self.current_piece = Some(piece);
    }

    fn rotate_piece_right(&mut self) {
        if let Some(mut piece) = self.current_piece {
            let size = piece.size;
            let mut rotated_piece = [[0u8; 4]; 4];

            for (row_idx, row) in piece.shape.iter().enumerate().take(size) {
                for (col_idx, &cell) in row.iter().enumerate().take(size) {
                    rotated_piece[col_idx][size - 1 - row_idx] = cell;
                }
            }

            if !self.check_collision(self.current_x, self.current_y, &rotated_piece) {
                piece.shape = rotated_piece;
                self.current_piece = Some(piece);
            }
        }
    }

    fn rotate_piece_left(&mut self) {
        if let Some(mut piece) = self.current_piece {
            let size = piece.size;
            let mut rotated_piece = [[0u8; 4]; 4];

            for (row_idx, row) in piece.shape.iter().enumerate().take(size) {
                for (col_idx, &cell) in row.iter().enumerate().take(size) {
                    rotated_piece[col_idx][size - 1 - row_idx] = cell;
                }
            }

            if !self.check_collision(self.current_x, self.current_y, &rotated_piece) {
                piece.shape = rotated_piece;
                self.current_piece = Some(piece);
            }
        }
    }

    fn place_piece(&mut self) {
        if self.current_piece.is_none() {
            return;
        }

        let piece = self.current_piece.as_ref().unwrap().shape;

        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, &cell_value) in row.iter().enumerate() {
                if cell_value != 0 {
                    let board_x = self.current_x + col_idx as i32;
                    let board_y = self.current_y + row_idx as i32;

                    if board_y >= 0
                        && board_y < self.height as i32
                        && board_x >= 0
                        && board_x < self.width as i32
                    {
                        self.board[board_y as usize][board_x as usize] = self.current_color;
                    }
                }
            }
        }

        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;

        for row in (0..self.height).rev() {
            let is_full_line = self.board[row].iter().all(|&cell| cell != 0);

            if is_full_line {
                lines_cleared += 1;

                for row_move in (1..=row).rev() {
                    self.board[row_move] = self.board[row_move - 1];
                }
                self.board[0].fill(0);
            }
        }

        if lines_cleared > 0 {
            self.score += lines_cleared * lines_cleared * 100
        }
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Left | KeyCode::Char('a') => {
                        if !self.check_collision(
                            self.current_x - 1,
                            self.current_y,
                            &self.current_piece.as_ref().unwrap().shape,
                        ) {
                            self.current_x -= 1;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        if !self.check_collision(
                            self.current_x + 1,
                            self.current_y,
                            &self.current_piece.as_ref().unwrap().shape,
                        ) {
                            self.current_x += 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        if !self.check_collision(
                            self.current_x,
                            self.current_y + 1,
                            &self.current_piece.as_ref().unwrap().shape,
                        ) {
                            self.current_y += 1;
                        } else {
                            self.place_piece();
                        }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        while !self.check_collision(
                            self.current_x,
                            self.current_y + 1,
                            &self.current_piece.as_ref().unwrap().shape,
                        ) {
                            self.current_y += 1;
                        }
                        self.place_piece();
                    }
                    KeyCode::Char('w') => self.rotate_piece_right(),
                    KeyCode::Esc => process::exit(1),
                    _ => (),
                }
            }
        }
    }

    pub fn run(&mut self) -> Result<i32, &'static str> {
        let mut last_tick = Instant::now();
        let mut drop_tick = Instant::now();
        let tick_rate = Duration::from_millis(8);
        let drop_rate = Duration::from_millis(500);

        let desk = Draw::new();

        self.new_game();

        while !self.game_over {
            self.handle_input();

            if last_tick.elapsed() >= tick_rate {
                desk.draw(
                    self.board,
                    &self.current_piece.as_ref().unwrap().shape,
                    self.score,
                    self.current_color,
                    self.current_x,
                    self.current_y,
                );

                last_tick = Instant::now();
            }

            if drop_tick.elapsed() >= drop_rate {
                if self.current_piece.is_none() {
                    continue;
                };

                if !self.check_collision(
                    self.current_x,
                    self.current_y + 1,
                    &self.current_piece.as_ref().unwrap().shape,
                ) {
                    self.current_y += 1;
                } else {
                    self.place_piece();
                }

                drop_tick = Instant::now();
            }
        }

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_works() {
        let mut game: Game<10, 20> = Game::new();

        crossterm::terminal::enable_raw_mode().expect("Couldn't turn on raw mode");

        let r = game.run().unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

        crossterm::terminal::disable_raw_mode().expect("Couldn't turn on raw mode");

        process::exit(r);
    }
}
