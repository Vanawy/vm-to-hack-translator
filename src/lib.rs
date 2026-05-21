use regex::Regex;
use crate::command::Command;

mod segment;
mod command;

pub fn translate(input: String) -> String {
    let re = Regex::new(r"\s+").expect("Can't parse regex");

    let lines: Vec<String> = input
        .lines()
        .map(|s| s.trim().to_lowercase())
        .map(|s| re.replace_all(s.as_str(), " ").to_string())
        .filter(|s| !(s.is_empty() || s.starts_with("//"))
        )
        .map(|s| (s.clone(), s.parse::<Command>()))
        .map(|(source, cmd)| format!("{:?} - {:?}", source,  cmd))
        .map(|s| s.into())
        .collect();

    println!("{}", lines.join("\n"));

    todo!()
}