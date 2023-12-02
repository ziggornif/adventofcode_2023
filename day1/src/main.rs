use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn read_input_file(input: String) -> Result<BufReader<File>> {
    let file = File::open(input)?;
    Ok(BufReader::new(file))
}

fn dummy_parser(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        result.push(c);
        result = replace_worded_numbers(result);
    }

    let mut reversed = String::new();
    let reversed_input: String = input.chars().rev().collect();

    for c in reversed_input.chars() {
        reversed.insert(0, c);
        reversed = replace_worded_numbers(reversed);
    }

    format!("{}{}", result, reversed)
}

fn replace_worded_numbers(input: String) -> String {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut result = input.clone();
    for (key, val) in numbers {
        result = result.replace(key, &*val.to_string());
    }

    result
}

fn calibration(input: String) -> Result<i32> {
    let reader = read_input_file(input)?;

    let mut sum = 0;

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let update_content = dummy_parser(&line_content);
                if let (Some(first), Some(last)) = (
                    update_content.chars().find(|c| c.is_ascii_digit()),
                    update_content.chars().rev().find(|c| c.is_ascii_digit()),
                ) {
                    let concatenated_str = format!("{}{}", first, last);

                    if let Ok(result) = concatenated_str.parse::<i32>() {
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
        assert_eq!(result, 142);
        Ok(())
    }

    #[test]
    fn should_calibrate_input_worded() -> Result<(), String> {
        let result = calibration("src/resources/test-input-2.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 281);
        Ok(())
    }

    #[test]
    fn should_calibrate_input_worded_2() -> Result<(), String> {
        let result = calibration("src/resources/test-input-3.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 78);
        Ok(())
    }
}
