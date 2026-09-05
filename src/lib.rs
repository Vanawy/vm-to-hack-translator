use crate::command::Command;
pub use crate::translator::Translator;
use regex::Regex;

mod command;
mod segment;
mod translator;

#[derive(Debug, PartialEq, Eq)]
pub struct TranslatorError {
    pub message: String,
}

pub fn translate(filename: String, input: String) -> Result<String, TranslatorError> {
    let re = Regex::new(r"\s+").expect("Can't parse regex");

    let mut translator = Translator::new(filename.clone());

    let lines: Vec<String> = input
        .lines()
        .enumerate()
        .map(|(n, s)| (n, s.trim().to_lowercase()))
        .map(|(n, s)| (n, re.replace_all(s.as_str(), " ").to_string()))
        .filter(|(_, s)| !(s.is_empty() || s.starts_with("//")))
        .map(|(n, s)| {
            let command = s.parse::<Command>().map_err(|error| TranslatorError {
                message: format!("line {}: {:?}", n + 1, error),
            })?;

            Ok(translator.code(command))
        })
        .collect::<Result<Vec<Vec<String>>, TranslatorError>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(lines.join("\n"))
}
