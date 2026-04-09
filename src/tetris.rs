struct Piece {
    shape: &'static [&'static [u8]]
}

pub fn run() -> Result<usize, &'static str> {
    const BOARD_WIDTH: usize = 10;
    const BOARD_HEIGHT: usize = 20;

    let mut board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

    let mut current_piece: Vec<u8> = Vec::new();

    let pieces: &[Piece] = &[
        Piece { shape: &[&[1, 1, 1, 1]] }
    ];

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
