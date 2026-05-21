use std::str::FromStr;
use crate::segment::Segment;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Stack {
        operation: StackOperation,
        segment: Segment,
        offset: u16
    },
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackOperation {
    Push,
    Pop
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticCommand {
    Add, Sub, Neg, Eq, Gt, Lt,
    And, Or, Not
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
                Err(_) => Err(CommandParseError)
            },
            3 => {
                let op = components[0].parse::<StackOperation>();
                let segment = components[1].parse::<Segment>();
                let offset = components[2].parse::<u16>();

                if let (Ok(op), Ok(segment), Ok(offset)) = (op, segment, offset) {
                    Ok(Command::Stack {
                        operation: op,
                        segment,
                        offset
                    })
                }
                else {
                    Err(CommandParseError)
                }
            },
            _ => Err(CommandParseError)
        }

    }
}

impl FromStr for ArithmeticCommand {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
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
            _ => Err(CommandParseError)
        }
    }
}

impl FromStr for StackOperation {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "push" => Ok(StackOperation::Push),
            "pop" => Ok(StackOperation::Pop),
            _ => Err(CommandParseError)
        }
    }
}
