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

    pub fn draw_bottom(&self, width: usize) {
        print!("{0}", self.bottom_left);
        for _ in 0..(width*2) {
            print!("{0}", self.bottom)
        }
        println!("{0}", self.bottom_right);
    } 

    pub fn draw_top(&self, width: usize) {
        print!("{0}", self.top_left);
        for _ in 0..(width*2) {
            print!("{0}", self.top)
        }
        println!("{0}", self.top_right);
    } 

    pub fn draw_center(&self, width: usize) {
        
    }
}
