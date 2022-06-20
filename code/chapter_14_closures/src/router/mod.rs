use std::collections::HashMap;

pub mod request;
pub mod response;
pub use request::Request;
pub use response::Response;

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;
pub struct BasicRouter<BoxedCallback>
where
    BoxedCallback: Fn(&request::Request) -> response::Response,
{
    routes: HashMap<String, BoxedCallback>,
}

impl<BoxedCallback> BasicRouter<BoxedCallback>
where
    BoxedCallback: Fn(&request::Request) -> response::Response,
{
    pub fn new() -> BasicRouter<BoxedCallback> {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, url: &str, callback: BoxedCallback) {
        self.routes.insert(url.to_string(), callback);
    }
}
