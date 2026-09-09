pub use crate::translator::Translator;

mod command;
mod segment;
mod translator;

#[derive(Debug, PartialEq, Eq)]
pub struct TranslatorError {
    pub message: String,
}

struct Line {
    content: String,
    number: usize,
}

pub fn translate(filename: String, input: String) -> Result<String, TranslatorError> {
    let mut translator = Translator::new(filename.clone());

    let lines: Vec<String> = input
        .lines()
        .enumerate()
        .map(|(n, s)| Line {
            content: remove_comment(&s.trim().to_lowercase()),
            number: n + 1,
        })
        .filter(|l| !l.content.is_empty())
        .map(|line| {
            let command = line.content.parse().map_err(|error| TranslatorError {
                message: format!("line {}: {:?}", line.number, error),
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
