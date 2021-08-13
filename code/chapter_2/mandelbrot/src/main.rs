mod cmd_parser;
mod computation;
mod rend;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <file> <pixels> <upper_left> <lower_right>",
            args[0]
        );
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.9,0.20",
            args[0]
        );
        std::process::exit(1);
    }
    let bounds = cmd_parser::parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left =
        cmd_parser::parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right =
        cmd_parser::parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    rend::render(&mut pixels, bounds, upper_left, lower_right);
    rend::write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
