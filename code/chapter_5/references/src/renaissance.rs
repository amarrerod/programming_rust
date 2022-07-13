use std::collections::HashMap;

pub type Table = HashMap<String, Vec<String>>;

pub fn show_work(table: &Table) {
    for (artist, works) in table {
        println!("Works of {}", artist);
        for work in works {
            println!("- {}", work);
        }
    }
}

pub fn sort_work(table: &mut Table) {
    for (_artist, works) in table {
        works.sort()
    }
}
