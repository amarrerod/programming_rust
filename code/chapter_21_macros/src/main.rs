mod json;
use json::Json;
fn main() {
    println!("Hello, world!");

    let width = 4.0;
    let desc = json!(
        [
            {
                "width": width,
                "height": (width * 9.0 / 4.0)
            }
        ]);
    println!("The variable is: {:#?}", desc);
}
