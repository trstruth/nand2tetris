use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::char;

use crate::code::{dest_lookup, jump_lookup, cmp_lookup};

pub struct Parser {
    lines: Lines<BufReader<File>>,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let buffered_reader = BufReader::new(file);
        let lines = buffered_reader.lines();

        Ok(Parser { lines })
    }
}

impl Iterator for Parser {
    type Item = Result<Command, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| match line {
            Ok(s) => Command::new(&s),
            Err(e) => Err(e),
        })
    }
}

pub struct Command {
    pub kind: CommandType,
}

impl Command {
    pub fn new(raw_line: &str) -> Result<Self, Error> {
        Ok(Self {
            kind: Self::parse_command_type(raw_line)?,
        })
    }

    pub fn parse_command_type(raw_line: &str) -> Result<CommandType, Error> {
        let mut line_string: String = raw_line.chars().filter(|c| !c.is_whitespace()).collect();

        if line_string.contains("//") {
            let comment_offset = line_string.find("//").unwrap();
            line_string.truncate(comment_offset);
        }

        if line_string.is_empty() {
            return Ok(CommandType::Empty);
        }

        if raw_line.chars().next().unwrap() == '@' {
            return Ok(CommandType::A {
                symbol: line_string.drain(1..).collect(),
            });
        }

        if line_string.contains("=") && line_string.contains(";") {
            let equals_offset = line_string.find('=').unwrap();
            let dest = line_string.drain(..equals_offset).collect();
            let semicolon_offset = line_string.find(';').unwrap();
            let comp = line_string.drain(1..semicolon_offset).collect();
            let jump = line_string.drain(2..).collect();

            return Ok(CommandType::C { dest, comp, jump });
        } else if line_string.contains("=") {
            let equals_offset = line_string.find('=').unwrap();
            let dest = line_string.drain(..equals_offset).collect();
            let comp = line_string.drain(1..).collect();
            return Ok(CommandType::C {
                dest,
                comp,
                jump: "".to_owned(),
            });
        } else if line_string.contains(";") {
            let semicolon_offset = line_string.find(';').unwrap();
            let comp = line_string.drain(..semicolon_offset).collect();
            let jump = line_string.drain(1..).collect();
            return Ok(CommandType::C {
                dest: "".to_owned(),
                comp,
                jump,
            });
        }

        return Ok(CommandType::L {
            symbol: line_string.drain(..).collect(),
        });
    }

    pub fn to_code(&self) -> String {
        let mut binary_line = [0; 16];

        match &self.kind {
            CommandType::A { symbol } => {
                let value = symbol.parse::<u32>().unwrap();
                let value_bin_repr = format!("{:b}", value);

                let mut binary_line_idx = binary_line.len() - 1;
                for binary_char in value_bin_repr.chars().rev() {
                    binary_line[binary_line_idx] = binary_char.to_digit(10).unwrap() as u8;
                    binary_line_idx -= 1;
                }
            }
            CommandType::C { dest, comp, jump } => {
                binary_line[0] = 1;
                binary_line[1] = 1;
                binary_line[2] = 1;

                let comp_code_str = cmp_lookup(comp).unwrap();
                let dest_code_str = dest_lookup(dest).unwrap();
                let jump_code_str = jump_lookup(jump).unwrap();

                let binary_line_string = format!("111{}{}{}", comp_code_str, dest_code_str, jump_code_str);

                for (idx, c) in binary_line_string.chars().enumerate() {
                    binary_line[idx] = c.to_digit(10).unwrap() as u8;
                }
            },
            _ => unimplemented!(),
        }

        binary_line.iter().map(|d| {char::from_digit(*d as u32, 10).unwrap()}).collect()
    }
}

pub enum CommandType {
    A {
        symbol: String,
    },
    C {
        dest: String,
        comp: String,
        jump: String,
    },
    L {
        symbol: String,
    },
    Empty,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_a_command() {
        let raw_line = "@123";
        let a = Command::parse_command_type(&raw_line);
        assert_eq!(
            true,
            match a.unwrap() {
                CommandType::A { symbol } => {
                    symbol == "123".to_string()
                }
                _ => false,
            }
        )
    }

    #[test]
    fn test_parse_c_command() {
        let raw_line = "dest=comp;jump";
        let c = Command::parse_command_type(&raw_line);
        assert_eq!(
            true,
            match c.unwrap() {
                CommandType::C { dest, comp, jump } => {
                    dest == "dest" && comp == "comp" && jump == "jump"
                }
                _ => false,
            }
        )
    }

    #[test]
    fn test_parse_c_command_no_jump() {
        let raw_line = "dest=comp";
        let c = Command::parse_command_type(&raw_line);
        assert_eq!(
            true,
            match c.unwrap() {
                CommandType::C { dest, comp, jump } => {
                    dest == "dest" && comp == "comp" && jump == ""
                }
                _ => false,
            }
        )
    }

    #[test]
    fn test_parse_c_command_no_dest() {
        let raw_line = "comp;jump";
        let c = Command::parse_command_type(&raw_line);
        assert_eq!(
            true,
            match c.unwrap() {
                CommandType::C { dest, comp, jump } => {
                    dest == "" && comp == "comp" && jump == "jump"
                }
                _ => false,
            }
        )
    }

    #[test]
    fn test_parse_l_command_with_symbol() {
        let raw_line = "123";
        let l = Command::parse_command_type(&raw_line);
        assert_eq!(
            true,
            match l.unwrap() {
                CommandType::L { symbol } => {
                    symbol == "123".to_string()
                }
                _ => false,
            }
        )
    }

    #[test]
    fn test_a_command_with_value() {
        let raw_line = "@100";
        let expected_binary = "0000000001100100";
        let actual_binary = Command::new(raw_line).unwrap().to_code();

        assert_eq!(expected_binary, actual_binary);
    }

    #[test]
    fn test_empty_line() {
        let raw_line = "// this is a comment";
        let parsed_command = Command::new(raw_line).unwrap();

        assert_eq!(true, match parsed_command.kind {
            CommandType::Empty => true,
            _ => false,
        });
    }

    #[test]
    fn test_multiple_c_commands() {
        let mut testbed: HashMap<&str, &str> = HashMap::new();
        testbed.insert("MD=M;JEQ", "1111110000011010");
        testbed.insert("D=M", "1111110000010000" );
        testbed.insert("M=D+M", "1111000010001000");
        testbed.insert("0;JMP", "1110101010000111");
        testbed.insert("D;JGT", "1110001100000001");

        for (raw_line, expected_binary) in testbed {
            let actual_binary = Command::new(raw_line).unwrap().to_code();
            assert_eq!(expected_binary, actual_binary);
        }
    }
}
