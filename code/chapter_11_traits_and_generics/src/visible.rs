pub trait Visible {
    fn draw(&self);

    fn in_origin(&self) -> bool;
}
