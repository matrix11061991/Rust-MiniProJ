use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let mut input_file = BufReader::new(File::open("input.txt")?);
    let mut output_file = BufWriter::new(File::create("output.txt")?);
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    output_file.write_all(&buffer)?;
    Ok(())
}
