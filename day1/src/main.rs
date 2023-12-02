use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn read_input_file(input: String) -> Result<BufReader<File>> {
    let file = File::open(input)?;
    Ok(BufReader::new(file))
}

fn calibration(input: String) -> Result<i32> {
    let reader = read_input_file(input)?;

    let mut sum = 0;

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                if let (Some(first), Some(last)) = (
                    line_content.chars().find(|c| c.is_ascii_digit()),
                    line_content.chars().rev().find(|c| c.is_ascii_digit()),
                ) {
                    let concatenated_str = format!("{}{}", first, last);

                    if let Ok(result) = concatenated_str.parse::<i32>() {
                        println!("Parsed Result: {}", result);
                        sum += result;
                    } else {
                        println!("Error parsing to numeric value");
                    }
                } else {
                    println!("No digits found in the line");
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

fn main() -> Result<()> {
    println!("Hello advent of code day 1 !");

    let sum = calibration("day1/src/resources/input.txt".to_owned())?;
    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calibration;

    #[test]
    fn should_calibrate_input() -> Result<(), String> {
        let result = calibration("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 228);
        Ok(())
    }
}
