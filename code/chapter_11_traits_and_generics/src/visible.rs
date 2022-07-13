pub trait Visible {
    fn draw(&self);

    fn in_origin(&self) -> bool;
}

pub enum Direction {
    North,
    South,
    West,
    East,
}

pub trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}
