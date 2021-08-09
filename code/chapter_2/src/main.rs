
use std::str::FromStr;
use std::env;
mod calculate_gcd;

fn main() {
    // let mut numbers = Vec::new();
    // for arg in env::args().skip(1) {
    //     numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    // }
    let numbers : Vec<u64> = env::args().skip(1).map(|x| u64::from_str(&x).expect("Error parsing argument")).collect();
    if numbers.len() == 0 {
        eprintln!("Usage: gcd <number>");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = calculate_gcd::gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
