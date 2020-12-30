use crate::parser::{Command, Parser};
use crate::writer::Writer;
use std::convert::{TryFrom, TryInto};
use std::io::{Error, ErrorKind};

pub struct Translator {
    lines: Vec<String>,
    parser: Parser,
    writer: Writer,
}

impl Translator {
    pub fn new<T, V>(input_iter: T) -> Result<Self, Error>
    where
        T: Iterator<Item = V>,
        V: TryInto<String>,
    {
        let mut lines: Vec<String> = Vec::new();
        for line in input_iter {
            match line.try_into() {
                Ok(mut l) => {
                    if let Some(comment_index) = l.find("//") {
                        l = l.drain(..comment_index).collect();
                    }

                    // trim leading and trailing whitespace
                    l = l.trim().to_string();

                    // ignore the line if it's empty
                    if l.is_empty() {
                        continue;
                    }

                    lines.push(l)
                }
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "failed to convert input iterable to string",
                    ))
                }
            };
        }

        let parser = Parser {};
        let writer = Writer {};

        Ok(Translator {
            lines,
            parser,
            writer,
        })
    }

    pub fn output(&self) -> Result<(), Error> {
        for line in &self.lines {
            let s: &str = line;
            let command = Command::try_from(s)?;
            println!("{:?}", command);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace_removal() {
        let input_lines: Vec<String> = vec![
            "// copyright: acme industries".into(),
            "".into(),
            "   ".into(),
            "push constant 1".into(),
            "push constant 3 // pushes the constant 3".into(),
        ];

        let expected_output_lines: Vec<String> =
            vec!["push constant 1".into(), "push constant 3".into()];

        let t = Translator::new(input_lines.iter()).unwrap();

        assert_eq!(t.lines, expected_output_lines);
    }
}
