use crate::command::Command;
use regex::Regex;

mod command;
mod segment;

pub fn translate(input: String) -> String {
    let re = Regex::new(r"\s+").expect("Can't parse regex");

    let lines: Vec<String> = input
        .lines()
        .map(|s| s.trim().to_lowercase())
        .map(|s| re.replace_all(s.as_str(), " ").to_string())
        .filter(|s| !(s.is_empty() || s.starts_with("//")))
        .map(|s| (s.clone(), s.parse::<Command>()))
        .map(|(source, cmd)| format!("{:?}\n{:?}\n", source, cmd))
        .map(|s| s.into())
        .collect();

    println!("{}", lines.join("\n"));

    todo!()
}
