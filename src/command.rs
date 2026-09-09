use crate::segment::Segment;
use std::fmt::{Display, Error};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Stack {
        operation: StackOperation,
        segment: Segment,
        index: u16,
    },
    Arithmetic(ArithmeticCommand),
    BranchingOperation {
        operation: BranchingOperation,
        label: Label,
    },
    Return,
    Function {
        statement: FunctionStatement,
        name: String,
        n_args: u16,
    },
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Stack {
                operation,
                segment,
                index,
            } => {
                write!(f, "{:?} {:?} {}", operation, segment, index)
            }
            Command::Arithmetic(command) => write!(f, "{:?}", command),
            Command::BranchingOperation { operation, label } => {
                write!(f, "{:?} {}", operation, label)
            }
            Command::Function {
                statement,
                name,
                n_args,
            } => write!(f, "{:?} {} {}", statement, name, n_args),
            Command::Return => write!(f, "return"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackOperation {
    Push,
    Pop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticCommand {
    Add,
    Subtract,
    Negate,
    Equals,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BranchingOperation {
    Label,
    Goto,
    IfGoto,
}

pub type Label = String;

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionStatement {
    Call,
    Declaration,
    Return,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandParseError {
    command: String,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || CommandParseError {
            command: s.to_owned(),
        };

        let mut tokens = s.split_whitespace();
        match (tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
            (Some(a), None, None, None) => {
                if let Ok(command) = a.parse() {
                    Ok(Command::Arithmetic(command))
                } else if let Ok(FunctionStatement::Return) = a.parse() {
                    Ok(Command::Return)
                } else {
                    Err(err())
                }
            }
            (Some(a), Some(b), None, None) => Ok(Command::BranchingOperation {
                operation: a.parse()?,
                label: b.into(),
            }),
            (Some(a), Some(b), Some(c), None) => {
                if let Ok(operation) = a.parse() {
                    Ok(Command::Stack {
                        operation: operation,
                        segment: b.parse().map_err(|_| err())?,
                        index: c.parse().map_err(|_| err())?,
                    })
                } else if let Ok(statement) = a.parse() {
                    Ok(Command::Function {
                        statement,
                        name: b.into(),
                        n_args: c.parse().map_err(|_| err())?,
                    })
                } else {
                    Err(err())
                }
            }
            _ => Err(err()),
        }
    }
}

impl FromStr for ArithmeticCommand {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(ArithmeticCommand::Add),
            "sub" => Ok(ArithmeticCommand::Subtract),
            "neg" => Ok(ArithmeticCommand::Negate),
            "eq" => Ok(ArithmeticCommand::Equals),
            "gt" => Ok(ArithmeticCommand::GreaterThan),
            "lt" => Ok(ArithmeticCommand::LessThan),
            "and" => Ok(ArithmeticCommand::And),
            "or" => Ok(ArithmeticCommand::Or),
            "not" => Ok(ArithmeticCommand::Not),
            _ => Err(CommandParseError {
                command: s.to_owned(),
            }),
        }
    }
}

impl FromStr for StackOperation {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(StackOperation::Push),
            "pop" => Ok(StackOperation::Pop),
            _ => Err(CommandParseError {
                command: s.to_owned(),
            }),
        }
    }
}

impl FromStr for BranchingOperation {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "label" => Ok(BranchingOperation::Label {}),
            "goto" => Ok(BranchingOperation::Goto),
            "if-goto" => Ok(BranchingOperation::IfGoto),
            _ => Err(CommandParseError {
                command: s.to_owned(),
            }),
        }
    }
}

impl FromStr for FunctionStatement {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "call" => Ok(FunctionStatement::Call),
            "function" => Ok(FunctionStatement::Declaration),
            "return" => Ok(FunctionStatement::Return),
            _ => Err(CommandParseError {
                command: s.to_owned(),
            }),
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
                index: 1,
            },
            "pop argument 1".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Stack {
                operation: StackOperation::Push,
                segment: Segment::Constant,
                index: 36,
            },
            "push constant 36".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Stack {
                operation: StackOperation::Pop,
                segment: Segment::This,
                index: 6,
            },
            "pop this 6".parse::<Command>().unwrap()
        );
    }

    #[test]
    fn parse_arithmetic_commands() {
        assert_eq!(
            Command::Arithmetic(ArithmeticCommand::Add),
            "add".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Arithmetic(ArithmeticCommand::Subtract),
            "sub".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Arithmetic(ArithmeticCommand::Negate),
            "neg".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Arithmetic(ArithmeticCommand::LessThan),
            "lt".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::Arithmetic(ArithmeticCommand::GreaterThan),
            "gt".parse::<Command>().unwrap()
        );
    }

    #[test]
    fn parse_branching() {
        assert_eq!(
            Command::BranchingOperation {
                operation: BranchingOperation::Label,
                label: "LOOP_START".into()
            },
            "label LOOP_START".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::BranchingOperation {
                operation: BranchingOperation::IfGoto,
                label: "LOOP_START".into()
            },
            "if-goto LOOP_START".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::BranchingOperation {
                operation: BranchingOperation::Goto,
                label: "LOOP_START".into()
            },
            "goto LOOP_START".parse::<Command>().unwrap()
        );
    }

    #[test]
    fn parse_garbage() {
        for garbage in [
            "asdasdasd asdasdasd asdasdasd asdasdasd",
            "push constant 36 asdasdasd",
            "pop this this",
        ] {
            assert_eq!(
                Err(CommandParseError {
                    command: garbage.into()
                }),
                garbage.parse::<Command>()
            );
        }
    }
}
