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
    fn horizontal(&self) {
        println!("{self.left}");
        println!("{self.right}");
    } 
}
