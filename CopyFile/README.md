Here is an example of a simple program in Rust that copies the contents of one file to another file:
```rust
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
```
This program opens the file "input.txt" for reading and the file "output.txt" for writing. It reads the entire contents of "input.txt" into a buffer, and then writes the contents of the buffer to "output.txt".

This program can be run from the command line by compiling it with the Rust compiler, rustc, and then running the resulting executable file:
```bash
$ rustc copy.rs
$ ./copy
```
It's also possible to use the Rust build tool, cargo, to build and run the program.
```bash
$ cargo run --release
```
