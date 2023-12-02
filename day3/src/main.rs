use std::io::{BufRead, Error};
use utils::read_input_file;

fn process(input: String) -> Result<i32, Error> {
    let reader = read_input_file(input)?;
    Ok(1)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 3 !");

    let result = process("day3/src/resources/input.txt".to_owned())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn should_do_something() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 1);
        Ok(())
    }
}