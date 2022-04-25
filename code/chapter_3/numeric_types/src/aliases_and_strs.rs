pub type Bytes = Vec<u8>;

pub fn decode(_data: &Bytes) -> Result<u8, String> {
    return Err("Not found".to_string());
}

pub fn print_str(message: &str) -> Option<&str> {
    println!("{}", message);
    Some(message)
}
