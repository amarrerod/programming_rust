use std::io::Result;

pub trait StringSet {
    fn new() -> Self;
    fn from_slice(strings: [&str]) -> Self;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str) -> Result<()>;
}
