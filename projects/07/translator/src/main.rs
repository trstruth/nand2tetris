use std::env;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};

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

    let t = Translator::new(buffered_reader.lines().map(|x| x.unwrap()))?;

    t.output()?;

    Ok(())
}
