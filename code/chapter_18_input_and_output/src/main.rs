use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;

        if line.contains(target) {
            println!("{}", line);
        }
        if line.contains("quit()") {
            break;
        }
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");

    let file =
        "/Users/amarrero/Proyectos/programming_rust/code/chapter_18_input_and_output/src/main.rs";
    let f = File::open(file).unwrap();
    let k = grep("    let f = File::open(file).unwrap();", BufReader::new(f));
    let lines = BufReader::new(File::open(file).unwrap())
        .lines()
        .collect::<io::Result<Vec<String>>>();
    println!("{:?}", lines.unwrap());
}
