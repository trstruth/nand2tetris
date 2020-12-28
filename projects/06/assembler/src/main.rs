use assembler::assembler::Assembler;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: assembler <name>.asm",
        ));
    }

    let input_asm_file = &args[1];

    let mut a = Assembler::new(input_asm_file)?;

    let mut output: Vec<String> = Vec::new();

    for line in a.assemble()? {
        output.push(line);
    }

    let input_path = Path::new(input_asm_file);

    let input_file_name = match input_path.file_stem() {
        Some(name) => name.to_str().unwrap(),
        None => "out",
    };

    let output_file_path = format!("{}.hack", input_file_name);

    let mut output_file = File::create(output_file_path)?;
    output_file.write(output.join("\n").as_bytes())?;

    Ok(())
}
