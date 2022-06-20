#[derive(Clone)]
struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        println!("Dropping an Appelation with name: {}", self.name);
        if !self.nicknames.is_empty() {
            println!("Also known as {}", self.nicknames.join(", "));
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut app = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "King of the gods".to_string(),
        ],
    };
    println!("Before assigning");
    app = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at end of block");
}
