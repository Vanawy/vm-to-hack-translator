use crate::segment::Segment;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Stack {
        operation: StackOperation,
        segment: Segment,
        offset: u16,
    },
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackOperation {
    Push,
    Pop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandParseError;

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components = s.split(' ').collect::<Vec<&str>>();

        match components.len() {
            1 => match components[0].parse::<ArithmeticCommand>() {
                Ok(cmd) => Ok(Command::Arithmetic(cmd)),
                Err(_) => Err(CommandParseError),
            },
            3 => {
                let operation = components[0]
                    .parse::<StackOperation>()
                    .map_err(|_| CommandParseError)?;
                let segment = components[1]
                    .parse::<Segment>()
                    .map_err(|_| CommandParseError)?;
                let offset = components[2]
                    .parse::<u16>()
                    .map_err(|_| CommandParseError)?;

                Ok(Command::Stack {
                    operation,
                    segment,
                    offset,
                })
            }
            _ => Err(CommandParseError),
        }
    }
}

impl FromStr for ArithmeticCommand {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(ArithmeticCommand::Add),
            "sub" => Ok(ArithmeticCommand::Sub),
            "neg" => Ok(ArithmeticCommand::Neg),
            "eq" => Ok(ArithmeticCommand::Eq),
            "gt" => Ok(ArithmeticCommand::Gt),
            "lt" => Ok(ArithmeticCommand::Lt),
            "and" => Ok(ArithmeticCommand::And),
            "or" => Ok(ArithmeticCommand::Or),
            "not" => Ok(ArithmeticCommand::Not),
            _ => Err(CommandParseError),
        }
    }
}

impl FromStr for StackOperation {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(StackOperation::Push),
            "pop" => Ok(StackOperation::Pop),
            _ => Err(CommandParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stack_commands() {
        assert_eq!(
            Command::Stack {
                operation: StackOperation::Pop,
                segment: Segment::Argument,
                offset: 1,
            },
            "pop argument 1".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Stack {
                operation: StackOperation::Push,
                segment: Segment::Constant,
                offset: 36,
            },
            "push constant 36".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Stack {
                operation: StackOperation::Pop,
                segment: Segment::This,
                offset: 6,
            },
            "pop this 6".parse::<Command>().unwrap()
        );
    }
}
