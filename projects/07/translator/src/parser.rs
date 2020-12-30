use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

pub struct Parser {}

#[derive(Debug)]
pub struct Command {
    kind: CommandType,
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let kind = CommandType::try_from(value)?;

        Ok(Command { kind })
    }
}

#[derive(Debug)]
pub enum CommandType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Push(u32),
}

impl TryFrom<&str> for CommandType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut word_iter = value.split_whitespace();

        let operator = match word_iter.next() {
            Some(s) => s,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "syntax error: expected operator token",
                ))
            }
        };

        Ok(match operator {
            "add" => Self::Add,
            "sub" => Self::Sub,
            "neg" => Self::Neg,
            "eq" => Self::Eq,
            "gt" => Self::Gt,
            "lt" => Self::Lt,
            "and" => Self::And,
            "or" => Self::Or,
            "not" => Self::Not,
            "push" => {
                match word_iter.next() {
                    Some("constant") => (),
                    Some(_) => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "syntax error: expected \"constant\" after push",
                        ))
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "syntax error: expected more after \"push\"",
                        ))
                    }
                };
                let constant_value: u32 = match word_iter.next() {
                    Some(s) => match s.parse::<u32>() {
                        Ok(n) => n,
                        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e.to_string())),
                    },
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "syntax error: expected constant after \"push constant\"",
                        ))
                    }
                };

                Self::Push(constant_value)
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid operator: {}", value),
                ))
            }
        })
    }
}
