use regex::Regex;
use text_colorizer::*;

fn replace_all(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

pub fn replace(target: &str, replacement: &str, text: &str) -> String {
    match replace_all(target, replacement, text) {
        Ok(v) => return v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    }
}
