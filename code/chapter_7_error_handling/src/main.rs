use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn main() {
    println!("Hello, world!");
    let filename = "example.txt";
    let mut cwd = env::current_dir().unwrap();
    cwd.push(filename);
    let error = format!("Couldn't open file: {:?}", cwd);
    let mut file = BufReader::new(File::open(cwd).expect(&error));
    let numbers = read_numbers(&mut file).unwrap();
    println!("{:?}", numbers);
}
