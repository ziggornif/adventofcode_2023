use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn read_input_file(input: String) -> Result<BufReader<File>> {
    let file = File::open(input)?;
    Ok(BufReader::new(file))
}

fn sum_of_games(input: String, red: i32, green: i32, blue: i32) -> Result<i32> {
    let reader = read_input_file(input)?;

    let mut sum = 0;

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                // here the magic
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    Ok(sum)
}

fn main() -> Result<()> {
    println!("Hello advent of code day 1 !");

    let sum = sum_of_games("day2/src/resources/input.txt".to_owned(), 12, 13, 14)?;
    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::sum_of_games;

    #[test]
    fn should_get_games_sum() -> Result<(), String> {
        let result = sum_of_games("src/resources/test-input.txt".to_owned(), 12, 13, 14)
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 8);
        Ok(())
    }
}
