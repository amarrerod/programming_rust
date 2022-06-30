use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
mod jsons;

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

    let file = "";
    let f = File::open(file).unwrap();
    let k = grep("    let f = File::open(file).unwrap();", BufReader::new(f));
    let lines = BufReader::new(File::open(file).unwrap())
        .lines()
        .collect::<io::Result<Vec<String>>>();
    let resulting_lines = lines.unwrap();
    println!("{:?}", &resulting_lines);
    let temp_file = File::create("tmp.txt").unwrap();
    let mut writer = BufWriter::new(temp_file);
    writeln!(&mut writer, "{:#?}", &resulting_lines);
    jsons::create_data();
}
