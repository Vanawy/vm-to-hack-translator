use crate::command::Command;
pub use crate::translator::Translator;
use regex::Regex;
use std::process::exit;

mod command;
mod segment;
mod translator;

pub fn translate(filename: String, input: String) -> String {
    let re = Regex::new(r"\s+").expect("Can't parse regex");

    let mut translator = Translator::new(filename.clone());

    let lines: Vec<String> = input
        .lines()
        .enumerate()
        .map(|(n, s)| (n, s.trim().to_lowercase()))
        .map(|(n, s)| (n, re.replace_all(s.as_str(), " ").to_string()))
        .filter(|(_, s)| !(s.is_empty() || s.starts_with("//")))
        .map(|(n, s)| (n, s.parse::<Command>()))
        .map(|(n, c)| match c {
            Ok(c) => c,
            Err(err) => {
                eprintln!("{:?}: {}:{}:1", err, filename.clone(), n + 1);
                exit(1);
            }
        })
        .flat_map(|c| translator.code(c))
        .collect();

    format!("{}", lines.join("\n"))
}
