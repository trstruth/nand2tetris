use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

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

#[derive(Debug, PartialEq)]
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
                    format!("synatx error: invalid operator: {}", operator),
                ))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line_error() {
        let input_line = "";
        match Command::try_from(input_line) {
            Ok(_) => panic!("empty line should not parse to a command".to_owned()),
            Err(e) => assert_eq!(e.to_string(), "syntax error: expected operator token"),
        };
    }

    #[test]
    fn test_parse_add() {
        let input_line = "add";
        match Command::try_from(input_line) {
            Ok(c) => assert_eq!(c.kind, CommandType::Add),
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn test_parse_push() {
        let input_line = "push constant 42";
        match Command::try_from(input_line) {
            Ok(c) => match c.kind {
                CommandType::Push(42) => (),
                _ => panic!("expected CommandType::Push(42), got {:?}", c),
            },
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn test_parse_invalid_operator() {
        let input_line = "do something";
        match Command::try_from(input_line) {
            Ok(_) => panic!("\"do something\" should not have successfully parsed"),
            Err(e) => assert_eq!(e.to_string(), "synatx error: invalid operator: do"),
        };
    }
}
