use std::collections::HashMap;
#[derive(Default)]
pub struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(code: u32) -> Response {
        Response {
            code: code,
            ..Default::default()
        }
    }
}
