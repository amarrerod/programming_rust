#[derive(Debug)]
pub enum MyOrdering {
    Less,
    Equal,
    Greater,
}

pub fn compare(n: i32, m: i32) -> MyOrdering {
    if n < m {
        MyOrdering::Less
    } else if n > m {
        MyOrdering::Greater
    } else {
        MyOrdering::Equal
    }
}
