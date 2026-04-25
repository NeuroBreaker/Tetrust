use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    execute, queue,
};
use std::io::{self, stdout};

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

    pub fn draw_bottom(&self, width: usize) {
        let mut stdout = stdout();

        queue!(stdout, Print(self.bottom_left)).unwrap();
        for _ in 0..(width * 2) {
            queue!(stdout, Print(self.bottom)).unwrap();
        }
        queue!(stdout, Print(self.bottom_right)).unwrap();
    }

    pub fn draw_top(&self, width: usize) {
        let mut stdout = stdout();

        queue!(stdout, Print(self.top_left)).unwrap();
        for _ in 0..(width * 2) {
            queue!(stdout, Print(self.top)).unwrap();
        }
        queue!(stdout, Print(self.top_right), Print("\n\r")).unwrap();
    }

    pub fn draw_center<const W: usize, const H: usize>(&self, board: &[[u8; W]; H]) {
        let mut stdout = stdout();
        for row in board.iter() {
            print!("{0}", self.left);
            for &cell in row.iter() {
                if cell == 0 {
                    print!("  ");
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(self.get_color(cell)),
                        Print("██"),
                        ResetColor
                    ).unwrap();
                }
            }
            queue!(stdout, Print(self.right), Print("\n\r")).unwrap();
        }
    }

    pub fn draw<const W: usize, const H: usize>(
        &self,
        board: [[u8; W]; H],
        piece: &[[u8; 4]; 4],
        score: u128,
        color: u8,
        x: i32,
        y: i32,
    ) {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        queue!(stdout, MoveTo(0, 0)).unwrap();

        let mut draw_board = board;
        self.move_piece(&mut draw_board, piece, x, y, color);

        let width = W;
        self.draw_top(width);
        self.draw_center(&draw_board);
        self.draw_bottom(width);

        queue!(
            stdout,
            Print(format!("\n\r  Счёт: {}\n\r", score)),
            Print("\n\r  Управление:\n\r"),
            Print("  ← → или A D - движение\n\r"),
            Print("  ↑ или W - поворот\n\r"),
            Print("  ↓ или S - ускорить\n\r"),
            Print("  Пробел - сброс\n\r"),
            Print("  Esc - выход\n\r")
        ).unwrap();
    }
}
