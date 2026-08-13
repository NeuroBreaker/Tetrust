use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
};
use rand::random_range;
use std::{
    io::stdout,
    process,
    time::{Duration, Instant},
};

use crate::draw::Draw;

#[derive(Clone, Copy)]
pub struct Piece {
    pub shape: [[u8; 4]; 4],
    index: u8,
    pub size: usize,
}

impl Piece {
    pub fn new(piece: &[&[u8]], size: usize, index: u8) -> Self {
        let mut shape = [[0u8; 4]; 4];
        for row in 0..piece.len() {
            for col in 0..piece[0].len() {
                shape[row][col] = piece[row][col];
            }
        }

        Self { shape, size, index }
    }
}

pub struct Game<const W: usize, const H: usize> {
    pub board: [[u8; W]; H],
    pub width: usize,
    pub height: usize,
    pub pieces: [Piece; 7],
    pub pieces_buffer: [Piece; 3],
    pub current_piece: Option<Piece>,
    pub current_color: u8,
    pub current_x: i32,
    pub current_y: i32,
    pub score: u128,
    pub game_over: bool,
}

impl<const W: usize, const H: usize> Game<W, H> {
    pub fn new() -> Self {
        let pieces: [Piece; 7] = [
            Piece::new(&[&[1, 1, 1, 1]], 4, 0),
            Piece::new(&[&[1, 1], &[1, 1]], 2, 1),
            Piece::new(&[&[0, 1, 0], &[1, 1, 1]], 3, 2),
            Piece::new(&[&[0, 1, 1], &[1, 1, 0]], 3, 3),
            Piece::new(&[&[1, 1, 0], &[0, 1, 1]], 3, 4),
            Piece::new(&[&[1, 0, 0], &[1, 1, 1]], 3, 5),
            Piece::new(&[&[0, 0, 1], &[1, 1, 1]], 3, 6),
        ];

        let pieces_buffer = [Piece::new(&[&[0]], 0, 7); 3];

        Self {
            board: [[0u8; W]; H],
            width: W,
            height: H,
            pieces,
            pieces_buffer,
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
        if self.pieces_buffer[2].shape == [[0; 4]; 4] {
            for i in 0..3 {
                let piece_index = random_range(0..self.pieces.len());
                let piece = self.pieces[piece_index];
                self.pieces_buffer[i] = piece;
            }
        }

        let piece = self.pieces_buffer[0];

        for i in 0..self.pieces_buffer.len() {
            if i >= 2 { break; }
            self.pieces_buffer[i] = self.pieces_buffer[i + 1];
        }

        self.current_color = piece.index + 1;
        self.current_x = self.width as i32 / 2 - 2;
        self.current_y = -1;

        self.pieces_buffer[2] = self.pieces[random_range(0..self.pieces.len())];

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
                    rotated_piece[size - 1 - col_idx][row_idx] = cell;
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
            while self.board[row].iter().all(|&cell| cell != 0) {
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
        if event::poll(Duration::from_millis(10)).unwrap()
            && let Event::Key(key_event) = event::read().unwrap()
        {
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
                KeyCode::Up | KeyCode::Char('w') => self.rotate_piece_right(),
                KeyCode::Down | KeyCode::Char('s') => self.rotate_piece_left(),
                KeyCode::Char('j') => {
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
                KeyCode::Esc => process::exit(1),
                _ => (),
            }
        }
    }

    pub fn run(&mut self) -> Result<i32, &'static str> {
        let mut last_tick = Instant::now();
        let mut drop_tick = Instant::now();
        let tick_rate = Duration::from_millis(8);
        let drop_rate = Duration::from_millis(500);

        let desk = Draw::new();
        execute!(stdout(), Clear(ClearType::All)).unwrap();

        self.new_game();

        while !self.game_over {
            self.handle_input();

            if last_tick.elapsed() >= tick_rate {
                let _ = desk.draw(self);

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
