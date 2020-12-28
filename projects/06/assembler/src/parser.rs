use std::char;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Lines};

use crate::code::{cmp_lookup, dest_lookup, jump_lookup};
use crate::symbols::SymbolTable;

pub struct Parser {
    lines: Lines<BufReader<File>>,
    pub symbol_table: SymbolTable,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let buffered_reader = BufReader::new(file);
        let mut symbol_table = SymbolTable::new();

        let mut rom_addr = 0;
        for line in buffered_reader.lines() {
            match CommandType::try_from(&*line?)? {
                CommandType::L { symbol } => {
                    symbol_table.add_symbol(&symbol, Some(rom_addr));
                }
                CommandType::A { symbol: _ }
                | CommandType::C {
                    dest: _,
                    comp: _,
                    jump: _,
                } => {
                    rom_addr += 1;
                }
                CommandType::Empty => {
                    continue;
                }
            };
        }

        let file = File::open(filename)?;
        let buffered_reader = BufReader::new(file);
        let lines = buffered_reader.lines();

        Ok(Parser {
            lines,
            symbol_table,
        })
    }
}

impl Iterator for Parser {
    type Item = Result<Command, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| match line {
            Ok(s) => match CommandType::try_from(&*s) {
                Ok(CommandType::A { symbol }) => {
                    // if symbol parses, it's already an address literal
                    if let Ok(_) = symbol.parse::<u32>() {
                        return Ok(Command {
                            kind: CommandType::A { symbol },
                        });
                    }
                    // if symbol exists in symbol_table, replace it with corresponding address
                    if let Some(address) = self.symbol_table.get_symbol_address(&symbol) {
                        return Ok(Command {
                            kind: CommandType::A {
                                symbol: address.to_string(),
                            },
                        });
                    }
                    // symbol must be a new variable, treat it accordingly
                    let address = self.symbol_table.add_symbol(&symbol, None);
                    Ok(Command {
                        kind: CommandType::A {
                            symbol: address.to_string(),
                        },
                    })
                }
                Ok(kind) => Ok(Command { kind }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        })
    }
}

pub struct Command {
    pub kind: CommandType,
}

impl Command {
    pub fn new(s: &str) -> Result<Self, Error> {
        match CommandType::try_from(s) {
            Ok(kind) => Ok(Command { kind }),
            Err(e) => Err(e),
        }
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

                let binary_line_string =
                    format!("111{}{}{}", comp_code_str, dest_code_str, jump_code_str);

                for (idx, c) in binary_line_string.chars().enumerate() {
                    binary_line[idx] = c.to_digit(10).unwrap() as u8;
                }
            }
            _ => unimplemented!(),
        }

        binary_line
            .iter()
            .map(|d| char::from_digit(*d as u32, 10).unwrap())
            .collect()
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

impl TryFrom<&str> for CommandType {
    type Error = std::io::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut line: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        if line.contains("//") {
            let comment_offset = line.find("//").unwrap();
            line.truncate(comment_offset);
        }

        if line.is_empty() {
            return Ok(CommandType::Empty);
        }

        // A Command
        if line.chars().next().unwrap() == '@' {
            let symbol: String = line.drain(1..).collect();
            return Ok(CommandType::A { symbol });
        }

        // L Command
        if line.starts_with('(') && line.ends_with(')') {
            let symbol: String = line.drain(1..line.len() - 1).collect();
            return Ok(CommandType::L { symbol });
        }

        // C Command
        if line.contains("=") && line.contains(";") {
            let equals_offset = line.find('=').unwrap();
            let dest = line.drain(..equals_offset).collect();
            let semicolon_offset = line.find(';').unwrap();
            let comp = line.drain(1..semicolon_offset).collect();
            let jump = line.drain(2..).collect();

            return Ok(CommandType::C { dest, comp, jump });
        } else if line.contains("=") {
            let equals_offset = line.find('=').unwrap();
            let dest = line.drain(..equals_offset).collect();
            let comp = line.drain(1..).collect();
            return Ok(CommandType::C {
                dest,
                comp,
                jump: "".to_owned(),
            });
        } else if line.contains(";") {
            let semicolon_offset = line.find(';').unwrap();
            let comp = line.drain(..semicolon_offset).collect();
            let jump = line.drain(1..).collect();
            return Ok(CommandType::C {
                dest: "".to_owned(),
                comp,
                jump,
            });
        }

        Err(Error::new(
            ErrorKind::InvalidInput,
            format!("syntax error: \"{}\" could not be parsed", s),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_a_command() {
        let raw_line = "@123";
        let a = CommandType::try_from(raw_line);
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
        let c = CommandType::try_from(raw_line);
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
        let c = CommandType::try_from(raw_line);
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
        let c = CommandType::try_from(raw_line);
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
        let raw_line = "(MARKER)";
        let l = CommandType::try_from(raw_line);
        assert_eq!(
            true,
            match l.unwrap() {
                CommandType::L { symbol } => {
                    symbol == "MARKER".to_string()
                }
                _ => false,
            }
        )
    }

    #[test]
    fn test_a_command_with_value() {
        let raw_line = "@100";
        let expected_binary = "0000000001100100";
        let actual_binary = Command {
            kind: CommandType::try_from(raw_line).unwrap(),
        }
        .to_code();

        assert_eq!(expected_binary, actual_binary);
    }

    #[test]
    fn test_empty_line() {
        let raw_line = "// this is a comment";
        let parsed_command = Command {
            kind: CommandType::try_from(raw_line).unwrap(),
        };

        assert_eq!(
            true,
            match parsed_command.kind {
                CommandType::Empty => true,
                _ => false,
            }
        );
    }

    #[test]
    fn test_multiple_c_commands() {
        let mut testbed: HashMap<&str, &str> = HashMap::new();
        testbed.insert("MD=M;JEQ", "1111110000011010");
        testbed.insert("D=M", "1111110000010000");
        testbed.insert("M=D+M", "1111000010001000");
        testbed.insert("0;JMP", "1110101010000111");
        testbed.insert("D;JGT", "1110001100000001");

        for (raw_line, expected_binary) in testbed {
            let actual_binary = Command {
                kind: CommandType::try_from(raw_line).unwrap(),
            }
            .to_code();
            assert_eq!(expected_binary, actual_binary);
        }
    }
}
