use std::fs;
use text_colorizer::*;

pub fn read_file(filename: &String) -> String {
    match fs::read_to_string(&filename) {
        Ok(v) => return v,
        Err(e) => {
            eprintln!(
                "{} reading file '{}' : {:?}",
                "Error".red().bold(),
                filename,
                e
            );
            std::process::exit(1);
        }
    };
}

pub fn write_to_file(filename: &String, data: &String) {
    match fs::write(&filename, &data) {
        Ok(_) => return {},
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error".red().bold(),
                filename,
                e
            );
        }
    };
}
