use crossterm::{
    cursor::MoveTo,
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

    pub fn move_piece<const W: usize, const H: usize>(
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

                    if y >= 0 && y < board.len() as i32 && x >= 0 && x < board[0].len() as i32 {
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
        queue!(stdout, Print(self.top_left))?;
        for _ in 0..(width * 2) {
            queue!(stdout, Print(self.top))?;
        }
        queue!(stdout, Print(self.top_right), Print("\n\r"))?;

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
                    print!("  ");
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(self.get_color(cell)),
                        Print("██"),
                        ResetColor
                    )?;
                }
            }
            queue!(stdout, Print(self.right), Print("\n\r"))?;
        }

        Ok(())
    }

    pub fn draw<const W: usize, const H: usize>(
        &self,
        board: [[u8; W]; H],
        piece: &[[u8; 4]; 4],
        score: u128,
        color: u8,
        x: i32,
        y: i32,
    ) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(stdout, MoveTo(0, 0))?;

        let mut draw_board = board;
        self.move_piece(&mut draw_board, piece, x, y, color);

        let width = W;
        self.draw_top(&mut stdout, width)?;
        self.draw_center(&mut stdout, &draw_board)?;
        self.draw_bottom(&mut stdout, width)?;

        queue!(
            stdout,
            Print(format!("\n\r  Счёт: {}\n\r", score)),
            Print("\n\r  Управление:\n\r"),
            Print("  ← → или A D - движение\n\r"),
            Print("  ↑ или W - поворот вправо\n\r"),
            Print("  ↓ или S - поворот влево\n\r"),
            Print("  J - ускорение падения \n\r"),
            Print("  Пробел - сброс\n\r"),
            Print("  Esc - выход\n\r")
        )?;

        stdout.flush()?;

        Ok(())
    }
}
