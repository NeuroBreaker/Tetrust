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

    pub fn draw<const W: usize, const H: usize>(
        &self,
        board: &[[u8; W]; H],
        piece: &[[u8; 4]; 4],
        score: u128,
        color: u8,
        x: i32,
        y: i32,
    ) -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());

        let mut draw_board = *board;
        self.overlay_piece(&mut draw_board, piece, x, y, color);

        let width = W;
        self.draw_top(&mut handle, width)?;
        self.draw_center(&mut handle, &draw_board)?;
        self.draw_bottom(&mut handle, width)?;

        queue!(
            handle,
            Print(format!("\r\n  Счёт: {}\r\n", score)),
            Print("\r\n  Управление:\r\n"),
            Print("  ← → или A D - движение\r\n"),
            Print("  ↑ или W - поворот вправо\r\n"),
            Print("  ↓ или S - поворот влево\r\n"),
            Print("  J - ускорение падения \r\n"),
            Print("  Пробел - сброс\r\n"),
            Print("  Esc - выход\r\n")
        )?;

        handle.flush()?;

        Ok(())
    }
}
