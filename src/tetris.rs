use colored::{Colorize, Color};
use rand::random_range;

use crate::draw::Draw;

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
            game_over: false,
        }
    }

    fn check_collision(&self, x: i32, y: i32, piece: &[&[u8]]) -> bool {
        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, &cell_value) in row.iter().enumerate() {
                if cell_value != 0 {
                    let board_x = x + col_idx as i32;
                    let board_y = y + row_idx as i32;

                    if board_x < 0 || board_x >= self.width as i32 || board_y >= self.height as i32 {
                        return true;
                    }

                    if board_y >= 0 {
                        if self.board[board_y as usize][board_x as usize] != 0 {
                            return true;
                        }
                    }
                }
            }
        }

        false
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

    fn place_piece(&mut self) {
        if self.current_piece.is_none() {
            return
        }

        let piece = self.current_piece.as_ref().unwrap().shape;

        for (row_idx, row) in piece.iter().enumerate()  {
            for (col_idx, &cell_value) in row.iter().enumerate() {
                if cell_value != 0 {
                    let board_x = self.current_x + col_idx as i32;
                    let board_y = self.current_y + row_idx as i32;

                    if board_y >= 0 && board_y < self.height as i32
                       && board_x >= 0 && board_x < self.width as i32
                    {
                        self.board[board_y as usize][board_x as usize] = self.current_color;
                    }
                }
            }
        }

        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&self) {
    }

    fn get_color(&self, index: usize) -> Color {
        match index {
            1 => Color::Cyan,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Green,
            5 => Color::Red,
            6 => Color::Blue,
            7 => Color::BrightYellow,
            _ => Color::White,
        }
    }

    pub fn run(&self) -> Result<u8, &'static str> {
        let c = false;
        let desk = Draw::new();

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("{0}", "123".color(Color::Red));
            desk.draw_top(self.width);
            desk.draw_bottom(self.width);

            println!("{input}");

            if c {
                break;
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
        let game: Game<10, 20> = Game::new();
        assert_eq!(game.run(), Ok(0));
    }

    #[test]
    fn color() {
        println!("{0}", "123".color(Color::Red));
        assert_eq!(1, 1);
    }
}
