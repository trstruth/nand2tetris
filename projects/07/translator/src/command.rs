use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum Command {
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

impl Command {
    pub fn to_asm(&self, id: usize) -> String {
        let asm_commands: Vec<String> = match self {
            Self::Push(val) => vec![
                format!("@{}", val),
                "D=A".into(), // load the constant into the D register
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // set value at the top of the stack to contents of the D register
                "@SP".into(),
                "M=M+1".into(),
            ],
            Self::Add => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "M=M+D".into(), // store x+y on the stack
                "@SP".into(),
                "M=M+1".into(),
            ],
            Self::Sub => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "M=M-D".into(), // store x-y on the stack
                "@SP".into(),
                "M=M+1".into(),
            ],
            Self::Neg => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "M=-M".into(), // negate y in place
                "@SP".into(),
                "M=M+1".into(), // increment the stack ptr
            ],
            Self::Eq => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "D=M-D".into(), // store x-y in D
                format!("@EQUAL:{}", id),
                "D;JEQ".into(), // if the difference is 0, the numbers are equal, jump to EQUAL label
                "@0".into(),
                "D=A".into(), // store "false" (0) in D
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // store false at the head of the stack ptr
                format!("@END:{}", id),
                "0;JMP".into(), // jump to the end
                format!("(EQUAL:{})", id),
                "@1".into(),
                "D=-A".into(), // store "true" (-1) in D
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // store true where x used to be
                format!("(END:{})", id),
                "@SP".into(),
                "M=M+1".into(), // increment stack ptr
            ],
            Self::Gt => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "D=M-D".into(), // store x-y in D
                format!("@GREATERTHAN:{}", id),
                "D;JGT".into(), // if the difference is > 0, then x > y
                "@0".into(),
                "D=A".into(), // store "false" (0) in D
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // store false at the head of the stack ptr
                format!("@END:{}", id),
                "0;JMP".into(), // jump to the end
                format!("(GREATERTHAN:{})", id),
                "@1".into(),
                "D=-A".into(), // store "true" (-1) in D
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // store true where x used to be
                format!("(END:{})", id),
                "@SP".into(),
                "M=M+1".into(), // increment stack ptr
            ],
            Self::Lt => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "D=M-D".into(), // store x-y in D
                format!("@LESSTHAN:{}", id),
                "D;JLT".into(), // if the difference is < 0, then x < y
                "@0".into(),
                "D=A".into(), // store "false" (0) in D
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // store false at the head of the stack ptr
                format!("@END:{}", id),
                "0;JMP".into(), // jump to the end
                format!("(LESSTHAN:{})", id),
                "@1".into(),
                "D=-A".into(), // store "true" (-1) in D
                "@SP".into(),
                "A=M".into(),
                "M=D".into(), // store true where x used to be
                format!("(END:{})", id),
                "@SP".into(),
                "M=M+1".into(), // increment stack ptr
            ],
            Self::And => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "M=M&D".into(), // store x&y on the stack
                "@SP".into(),
                "M=M+1".into(),
            ],
            Self::Or => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "D=M".into(), // record y in D
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr again, SP now points at x
                "A=M".into(),
                "M=M|D".into(), // store x&y on the stack
                "@SP".into(),
                "M=M+1".into(),
            ],
            Self::Not => vec![
                "@SP".into(),
                "M=M-1".into(), // decrement the stack ptr, SP now points at y
                "A=M".into(),
                "M=!M".into(), // negate y in place
                "@SP".into(),
                "M=M+1".into(), // increment the stack ptr
            ],
        };

        asm_commands.join("\n")
    }
}

impl TryFrom<&str> for Command {
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
            Ok(c) => assert_eq!(c, Command::Add),
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn test_parse_push() {
        let input_line = "push constant 42";
        match Command::try_from(input_line) {
            Ok(c) => match c {
                Command::Push(42) => (),
                _ => panic!("expected Command::Push(42), got {:?}", c),
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
