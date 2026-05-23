use crate::command::Command;
pub use crate::translator::Translator;
use regex::Regex;

mod command;
mod segment;
mod translator;

pub fn translate(filename: String, input: String) -> String {
    let re = Regex::new(r"\s+").expect("Can't parse regex");

    let mut translator = Translator::new(filename);

    let lines: Vec<String> = input
        .lines()
        .map(|s| s.trim().to_lowercase())
        .map(|s| re.replace_all(s.as_str(), " ").to_string())
        .filter(|s| !(s.is_empty() || s.starts_with("//")))
        .map(|s| s.parse::<Command>())
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .flat_map(|c| translator.code(c))
        .collect();

    format!("{}", lines.join("\n"))
}
