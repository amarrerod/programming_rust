use crate::visible::Visible;

pub struct Player {
    pub height: u32,
    pub width: u32,
    pub x: u32,
    pub y: u32,
}

impl Player {
    pub fn new(height: u32, width: u32) -> Player {
        Player {
            height: height,
            width: width,
            x: 0,
            y: 0,
        }
    }
}

impl Visible for Player {
    fn draw(&self) {
        println!("The player is in ({}, {})", self.x, self.y);
    }

    fn in_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}
