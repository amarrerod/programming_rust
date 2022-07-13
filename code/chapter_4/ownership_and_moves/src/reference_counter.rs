use std::rc::Rc;

pub fn references() {
    let s: Rc<String> = Rc::new("Shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
}
