use std::fs::File;
use std::io::{BufReader, Result};

pub fn read_input_file(input: String) -> Result<BufReader<File>> {
    let file = File::open(input)?;
    Ok(BufReader::new(file))
}
