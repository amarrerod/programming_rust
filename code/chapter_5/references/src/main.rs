mod lifetimes;
mod renaissance;
mod safety;
use lifetimes::{test_string_table, testing_lifetimes};
use renaissance::{show_work, sort_work, Table};
use safety::{g, smallest, S};

fn artists() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "A salt cellar".to_string(),
        ],
    );
    println!("Pre-sorted table");
    show_work(&table);
    sort_work(&mut table);
    println!("Sorted table");
    show_work(&table);
}

fn assigning_references() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    if true {
        r = &y;
    }
    assert!(*r == 20);
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn main() {
    artists();
    assigning_references();
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    //borrowing_local();
    let x = 10;
    g(&x);
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    let z;
    {
        let x = 10;
        z = S { r: &x };
        assert_eq!(*z.r, 10);
    }
    testing_lifetimes();
    test_string_table();
}
