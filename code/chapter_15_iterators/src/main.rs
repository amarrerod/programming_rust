mod binary_tree;
mod bt_iterator;
mod range;
use binary_tree::*;
use rand::random;
use std::collections::HashMap;
use std::iter::{from_fn, once, repeat};
use std::str::FromStr;

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

fn using_flat_map() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United states", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Säo Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|c| &major_cities[c]) {
        println!("{}", city);
    }
}

fn range_example() {
    let mut pi = 0.0;
    let mut numerator = 1.0;
    for k in (range::I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);
    println!("{}", pi);
}

fn main() {
    let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect();

    //println!("{:?}", lengths);

    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );

    let text = "   ponies \n  giraffes\niguanas    \nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

    // Use of filter_map
    let other_text = "1\nfrond .25 289\n3.1415 estuary\n";
    let numbers: Vec<f64> = other_text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
        .map(|x| x.sqrt())
        .collect();
    println!("{:?}", numbers);
    using_flat_map();

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });
    for line in fizz_buzz {
        println!("{}", line);
    }

    let a = [5, 6, 7, 8, 9, 10];
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);
    range_example();

    let mut tree = BinaryTree::Empty;
    tree.add("jaeger");
    tree.add("robot");
    tree.add("droid");
    tree.add("mecha");
    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }

    assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);
}
