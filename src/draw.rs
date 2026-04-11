use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    execute,
};
use std::io::stdout;

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
        piece: &[&[u8]],
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

    pub fn draw_bottom(&self, width: usize) {
        print!("{0}", self.bottom_left);
        for _ in 0..(width * 2) {
            print!("{0}", self.bottom)
        }
        println!("{0}", self.bottom_right);
    }

    pub fn draw_top(&self, width: usize) {
        print!("{0}", self.top_left);
        for _ in 0..(width * 2) {
            print!("{0}", self.top)
        }
        println!("{0}", self.top_right);
    }

    pub fn draw_center<const W: usize, const H: usize>(&self, board: &[[u8; W]; H]) {
        for row in board.iter() {
            print!("{0}", self.left);
            for &cell in row.iter() {
                if cell == 0 {
                    print!("  ");
                } else {
                    execute!(
                        stdout(),
                        SetForegroundColor(self.get_color(cell)),
                        Print("██"),
                        ResetColor
                    )
                    .unwrap();
                    //print!("██");
                }
            }
            println!("{0}", self.right);
        }
    }

    pub fn draw<const W: usize, const H: usize>(
        &self,
        board: [[u8; W]; H],
        piece: &[&[u8]],
        score: u128,
        color: u8,
        x: i32,
        y: i32,
    ) {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
        let mut draw_board = board;
        self.move_piece(&mut draw_board, piece, x, y, color);

        let width = W;
        self.draw_top(width);
        self.draw_center(&draw_board);
        self.draw_bottom(width);

        println!("\n  Счёт: {score}");
        println!("\n  Управление:");
        println!("  ← → или A D - движение");
        println!("  ↑ или W - поворот");
        println!("  ↓ или S - ускорить");
        println!("  Пробел - сброс");
        println!("  Esc - выход");
    }
}
