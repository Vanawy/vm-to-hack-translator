use crate::command::Command;
pub use crate::translator::Translator;

mod command;
mod segment;
mod translator;

#[derive(Debug, PartialEq, Eq)]
pub struct TranslatorError {
    pub message: String,
}

pub fn translate(filename: String, input: String) -> Result<String, TranslatorError> {
    let mut translator = Translator::new(filename.clone());

    let lines: Vec<String> = input
        .lines()
        .enumerate()
        .map(|(n, s)| (n, s.trim().to_lowercase()))
        .map(|(n, s)| (n, remove_comment(&s)))
        .filter(|(_, s)| !s.is_empty())
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

fn remove_comment(input: &str) -> String {
    input
        .split_once("//")
        .map_or(input, |(command, _)| command)
        .trim()
        .into()
}

#[cfg(test)]
mod lib {
    use super::*;
    #[test]
    fn comments() {
        assert_eq!("abc", remove_comment("abc // asd"));
        assert_eq!("", remove_comment("// asd"));
        assert_eq!("abc", remove_comment("abc"));
    }
}
