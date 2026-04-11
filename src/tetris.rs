use rand::random_range;

use crate::draw::Draw;
use std::io::{self, Write};

#[derive(Clone)]
struct Piece {
    shape: &'static [&'static [u8]],
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
            Piece {
                shape: &[&[1, 1, 1, 1]],
            },
            Piece {
                shape: &[&[1, 1], &[1, 1]],
            },
            Piece {
                shape: &[&[0, 1, 0], &[1, 1, 1]],
            },
            Piece {
                shape: &[&[0, 1, 1], &[1, 1, 0]],
            },
            Piece {
                shape: &[&[1, 1, 0], &[0, 1, 1]],
            },
            Piece {
                shape: &[&[1, 0, 0], &[1, 1, 1]],
            },
            Piece {
                shape: &[&[0, 0, 1], &[1, 1, 1]],
            },
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

    fn new_game(&mut self) {
        self.game_over = false;
        self.board = [[0u8; W]; H];
        self.score = 0;
        self.spawn_piece();
    }

    fn spawn_piece(&mut self) {
        let piece_index = random_range(0..self.pieces.len());
        let piece = self.pieces[piece_index].clone();
        let piece_width = piece.shape[0].len();
        self.current_color = piece_index as u8 + 1;
        self.current_x = self.width as i32 / 2 - piece_width as i32 / 2;
        self.current_y = -1;

        if self.check_collision(self.current_x, self.current_y, piece.shape) {
            self.game_over = true;
        }
        self.current_piece = Some(piece);
    }

    //fn rotate_piece(&self, piece: &[&[u8]]) {
    //    let rows = piece.len();
    //    let cols = piece[0].len();
    //    let mut rotated_piece = piece;
    //    for (row_idx, row) in piece.iter().enumerate() {
    //        for (col_idx, cell) in row.iter().enumerate() {
    //            rotated_piece[col_idx][rows - 1 - row_idx] = piece[row_idx][col_idx];
    //        }
    //    }
    //}

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

    fn check_collision(&self, x: i32, y: i32, piece: &[&[u8]]) -> bool {
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

    pub fn run(&mut self) -> Result<u8, &'static str> {
        let desk = Draw::new();
        self.new_game();

        while !self.game_over {
            if self.current_piece.is_none() {
                continue;
            };
            if !self.check_collision(
                self.current_x,
                self.current_y + 1,
                self.current_piece.as_ref().unwrap().shape,
            ) {
                self.current_y += 1;
            } else {
                self.place_piece();
            }
            desk.draw(
                self.board,
                self.current_piece.as_ref().unwrap().shape,
                self.score,
                self.current_color,
                self.current_x,
                self.current_y,
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
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
        assert_eq!(game.run(), Ok(0));
    }
}
