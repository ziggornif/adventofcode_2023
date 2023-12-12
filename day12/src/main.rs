use std::io::Error;

fn process(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;
    Ok(0)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 12 !");
    let result = process("day12/src/resources/input.txt".to_owned())?;
    println!("Result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn should_get_p1_result() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 21);
        Ok(())
    }
}
