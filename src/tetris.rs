struct Piece {
    shape: &'static [&'static [u8]]
}

pub fn run() -> Result<usize, &'static str> {
    const BOARD_WIDTH: usize = 10;
    const BOARD_HEIGHT: usize = 20;

    let mut board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

    let mut current_piece: Vec<u8> = Vec::new();

    let pieces: &[Piece] = &[
        Piece { shape: &[&[1, 1, 1, 1]] },
        Piece { shape: &[&[1, 1], &[1, 1]] },
        Piece { shape: &[&[0, 1, 0], &[1, 1, 1]] },
        Piece { shape: &[&[0, 1, 1], &[1, 1, 0]] },
        Piece { shape: &[&[1, 1, 0], &[0, 1, 1]] },
        Piece { shape: &[&[1, 0, 0], &[1, 1, 1]] },
        Piece { shape: &[&[0, 0, 1], &[1, 1, 1]] },
    ];

    fn check_collision(x: u8, y: u8, piece: Piece) -> bool {
        for i in 0..10 /* piece.shape.len()*/ {

        }

        false
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_works() {
        assert_eq!(run(), Ok(0));
    }
}
