use colored::Color;
use rand::random_range;

#[derive(Clone)]
struct Piece {
    shape: &'static [&'static [u8]],
}

impl Piece {
    fn new() -> Self {
        Self {
            shape: &[&[1, 1, 1, 1]],
        }
    }
}

pub struct Game<const W: usize, const H: usize> {
    board: [[u8; W]; H],
    width: usize,
    height: usize,
    pieces: [Piece; 7],
    current_piece: Piece,
    current_color: Color,
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
            current_piece: Piece::new(),
            current_color: Color::White,
        }
    }

    fn check_collision(&self, x: usize, y: usize, piece: &[&[u8]]) -> bool {
        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, &cell_value) in row.iter().enumerate() {
                if cell_value != 0 {
                    let board_x = x + col_idx;
                    let board_y = y + row_idx;

                    if board_x < 0 || board_x >= self.width || board_y >= self.height {
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
        self.current_piece = self.pieces[piece_index].clone();
        self.current_color = self.get_color(piece_index);
    }

    fn get_color(&self, index: usize) -> Color {
        match index {
            0 => Color::Cyan,
            1 => Color::Yellow,
            2 => Color::Magenta,
            3 => Color::Green,
            4 => Color::Red,
            5 => Color::Blue,
            6 => Color::BrightYellow,
            _ => Color::White,
        }
    }

    pub fn run(&self) -> Result<u8, &'static str> {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("{input}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_works() {
        let game = Game::new();
        assert_eq!(game.run(), Ok(0));
    }
}
