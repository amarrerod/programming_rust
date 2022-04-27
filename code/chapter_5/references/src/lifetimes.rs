struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

pub fn testing_lifetimes() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
    println!("R is {}", r);
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    pub fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].to_uppercase().starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

pub fn test_string_table() {
    let table = StringTable {
        elements: vec![
            "Hola".to_string(),
            "Mundo".to_string(),
            "In".to_string(),
            "Rust".to_string(),
        ],
    };
    let s = table
        .find_by_prefix(&"hola".to_uppercase())
        .expect("No string found");
    println!("{}", s);
}
