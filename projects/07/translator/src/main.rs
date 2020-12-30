use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

use translator::Translator;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: translator <name>.vm",
        ));
    }

    let input_filepath_string = &args[1];

    let input_file = File::open(input_filepath_string)?;
    let buffered_reader = io::BufReader::new(input_file);

    let mut t = Translator::new(buffered_reader.lines().map(|x| x.unwrap()))?;

    t.translate()?;

    let input_path = Path::new(input_filepath_string);
    let input_filename = match input_path.file_stem() {
        Some(name) => name.to_str().unwrap(),
        None => "out",
    };

    let output_filepath = format!("{}.asm", input_filename);

    let mut output_file = File::create(output_filepath)?;
    output_file.write(t.output().as_bytes())?;

    Ok(())
}
