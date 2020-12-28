use crate::parser::{CommandType, Parser};

pub struct Assembler {
    parser: Parser,
}

impl Assembler {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let parser = Parser::new(filename)?;

        Ok(Assembler { parser })
    }

    pub fn assemble(&mut self) -> Result<Vec<String>, std::io::Error> {
        let mut output: Vec<String> = vec![];
        for parsed_line in &mut self.parser {
            let command = parsed_line?;
            match command.kind {
                CommandType::Empty | CommandType::L { symbol: _ } => {
                    continue;
                }
                CommandType::A { symbol: _ }
                | CommandType::C {
                    dest: _,
                    comp: _,
                    jump: _,
                } => {
                    output.push(command.to_code());
                }
            };
        }

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_assemble() {
        let mut test_asm_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_asm_filepath.push("../max/MaxL.asm");

        let mut a = Assembler::new(test_asm_filepath.to_str().unwrap()).unwrap();

        let assembled_output = a.assemble().unwrap();

        let expected_output = vec![
            "0000000000000000",
            "1111110000010000",
            "0000000000000001",
            "1111010011010000",
            "0000000000001010",
            "1110001100000001",
            "0000000000000001",
            "1111110000010000",
            "0000000000001100",
            "1110101010000111",
            "0000000000000000",
            "1111110000010000",
            "0000000000000010",
            "1110001100001000",
            "0000000000001110",
            "1110101010000111",
        ];

        assert_eq!(assembled_output, expected_output);
    }

    #[test]
    fn test_assemble_with_symbols() {
        let mut test_asm_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_asm_filepath.push("../rect/Rect.asm");

        let mut a = Assembler::new(test_asm_filepath.to_str().unwrap()).unwrap();

        let assembled_output = a.assemble().unwrap();

        let expected_output = vec![
            "0000000000000000",
            "1111110000010000",
            "0000000000010111",
            "1110001100000110",
            "0000000000010000",
            "1110001100001000",
            "0100000000000000",
            "1110110000010000",
            "0000000000010001",
            "1110001100001000",
            "0000000000010001",
            "1111110000100000",
            "1110111010001000",
            "0000000000010001",
            "1111110000010000",
            "0000000000100000",
            "1110000010010000",
            "0000000000010001",
            "1110001100001000",
            "0000000000010000",
            "1111110010011000",
            "0000000000001010",
            "1110001100000001",
            "0000000000010111",
            "1110101010000111",
        ];

        assert_eq!(assembled_output, expected_output);
    }
}
