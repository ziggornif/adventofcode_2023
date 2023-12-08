use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_input_file(input: String) -> Result<BufReader<File>, Error> {
    let file = File::open(input)?;
    Ok(BufReader::new(file))
}

pub fn read_input_file_as_vec(input: String) -> Result<Vec<String>, Error> {
    let reader = read_input_file(input)?;
    let lines_arr: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines_arr)
}
