use crate::tetris::{Game, Piece};
use crossterm::{
    cursor::{Hide, MoveTo},
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Write};

pub struct Draw {
    top: char,
    bottom: char,
    left: char,
    right: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
}

impl Draw {
    pub fn new() -> Self {
        Self {
            top: '═',
            bottom: '═',
            left: '║',
            right: '║',
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
        }
    }

    fn get_color(&self, index: u8) -> Color {
        match index {
            1 => Color::Cyan,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Green,
            5 => Color::Red,
            6 => Color::Blue,
            7 => Color::DarkYellow,
            _ => Color::White,
        }
    }

    pub fn overlay_piece<const W: usize, const H: usize>(
        &self,
        board: &mut [[u8; W]; H],
        piece: &[[u8; 4]; 4],
        current_x: i32,
        current_y: i32,
        color: u8,
    ) {
        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    let y = current_y + row_idx as i32;
                    let x = current_x + col_idx as i32;

                    if y >= 0 && y < H as i32 && x >= 0 && x < W as i32 {
                        board[y as usize][x as usize] = color;
                    }
                }
            }
        }
    }

    pub fn draw_bottom<W: Write>(&self, stdout: &mut W, width: usize) -> io::Result<()> {
        queue!(stdout, Print(self.bottom_left))?;
        for _ in 0..(width * 2) {
            queue!(stdout, Print(self.bottom))?;
        }
        queue!(stdout, Print(self.bottom_right))?;

        Ok(())
    }

    pub fn draw_top<W: Write>(&self, stdout: &mut W, width: usize) -> io::Result<()> {
        queue!(stdout, MoveTo(0, 0), Hide)?;

        queue!(stdout, Print(self.top_left))?;
        for _ in 0..(width * 2) {
            queue!(stdout, Print(self.top))?;
        }
        queue!(stdout, Print(self.top_right), Print("\r\n"))?;

        Ok(())
    }

    pub fn draw_center<W: Write, const WIDTH: usize, const HEIGHT: usize>(
        &self,
        stdout: &mut W,
        board: &[[u8; WIDTH]; HEIGHT],
    ) -> io::Result<()> {
        for row in board.iter() {
            queue!(stdout, Print(self.left))?;
            for &cell in row.iter() {
                if cell == 0 {
                    queue!(stdout, Print("  "))?;
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(self.get_color(cell)),
                        Print("██"),
                        ResetColor
                    )?;
                }
            }
            queue!(stdout, Print(self.right), Print("\r\n"))?;
        }

        Ok(())
    }

    pub fn draw_next_shapes<W: Write>(&self, stdout: &mut W, buffer: &[Piece]) -> io::Result<()> {
        let space = 24;
        let weight = 28;
        let mut height = 0;
        queue!(stdout, MoveTo(space, height), Hide)?;

        queue!(stdout, Print(self.top_left))?;
        for _ in 0..weight {
            queue!(stdout, Print(self.top))?;
        }
        queue!(stdout, Print(self.top_right), Print("\r\n"))?;

        height += 1;
        queue!(stdout, MoveTo(space, height), Hide)?;
        queue!(stdout, Print(self.left))?;
        for _ in 0..weight {
            queue!(stdout, Print(" "))?;
        }
        queue!(stdout, Print(self.right))?;

        for row_idx in 0..2 {
            height += 1;
            queue!(stdout, MoveTo(space, height), Hide)?;
            queue!(stdout, Print(self.left))?;
            queue!(stdout, Print(" "))?;
            
            for (idx, piece) in buffer.iter().enumerate() {
                if idx >= 3 { break; }
                for col in 0..4 {
                    if piece.shape[row_idx][col] == 0 {
                        queue!(stdout, Print("  "))?;
                    } else {
                        queue!(
                            stdout,
                            SetForegroundColor(self.get_color(piece.index + 1)),
                            Print("██"),
                            ResetColor
                        )?;
                    }
                }
                queue!(stdout, Print(" "))?;
            }

            queue!(stdout, Print(self.right), Print("\r\n"))?;
        }


        height += 1;
        queue!(stdout, MoveTo(space, height), Hide)?;
        queue!(stdout, Print(self.left))?;
        for _ in 0..weight {
            queue!(stdout, Print(" "))?;
        }
        queue!(stdout, Print(self.right))?;

        height += 1;
        queue!(stdout, MoveTo(space, height), Hide)?;
        queue!(stdout, Print(self.bottom_left))?;
        for _ in 0..weight {
            queue!(stdout, Print(self.bottom))?;
        }
        queue!(stdout, Print(self.bottom_right), Print("\r\n"))?;
        Ok(())
    }

    pub fn draw<const W: usize, const H: usize>(
        &self,
        tetris: &Game<W, H>,
    ) -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());

        let board: &[[u8; W]; H] = &tetris.board;
        let piece: &[[u8; 4]; 4] = &tetris.current_piece.as_ref().unwrap().shape;
        let score: u128 = tetris.score;
        let color: u8 = tetris.current_color;
        let x: i32 = tetris.current_x;
        let y: i32 = tetris.current_y;

        let mut draw_board = *board;
        self.overlay_piece(&mut draw_board, piece, x, y, color);

        let width = W;
        self.draw_next_shapes(&mut handle, &tetris.pieces_buffer)?;
        self.draw_top(&mut handle, width)?;
        self.draw_center(&mut handle, &draw_board)?;
        self.draw_bottom(&mut handle, width)?;

        let control_height = 15;
        let score_height = 7;
        let space = 26;
        queue!(
            handle,
            MoveTo(space, score_height), Hide,
            SetForegroundColor(Color::Green),
            Print(format!("Счёт: {}\r\n", score)),
            ResetColor,
            MoveTo(space, control_height), Hide,
            Print("Управление:\r\n"),
            MoveTo(space, control_height + 1), Hide,
            Print("← → - движение\r\n"),
            MoveTo(space, control_height + 2), Hide,
            Print("X - поворот вправо\r\n"),
            MoveTo(space, control_height + 3), Hide,
            Print("Z - поворот влево\r\n"),
            MoveTo(space, control_height + 4), Hide,
            Print("↓ - ускорение падения \r\n"),
            MoveTo(space, control_height + 5), Hide,
            Print("Пробел - жёсткое падение\r\n"),
            MoveTo(space, control_height + 6), Hide,
            Print("Esc - выход\r\n")
        )?;

        handle.flush()?;

        Ok(())
    }
}
