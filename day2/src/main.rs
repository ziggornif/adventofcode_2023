use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use regex::Regex;

fn read_input_file(input: String) -> Result<BufReader<File>> {
    let file = File::open(input)?;
    Ok(BufReader::new(file))
}

struct Count {
    color: String,
    count: i32,
}

fn extract_game_number(input: &str) -> i32 {
    let re = Regex::new(r"Game (\d+):").unwrap();
    if let Some(captures) = re.captures(&input) {
        if let Some(game_number) = captures.get(1) {
            return game_number.as_str().parse().unwrap();
        }
    }
    0 // Default value if not found
}

fn extract_counts(input: &str) -> Vec<Count> {
    let mut counts = Vec::new();
    let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    for captures in re.captures_iter(&input) {
        let count = captures[1].parse().unwrap();
        let color = captures[2].to_string();
        counts.push(Count { color, count });
    }

    counts
}

fn sum_of_games(input: String, red: i32, green: i32, blue: i32) -> Result<i32> {
    let reader = read_input_file(input)?;

    let mut sum = 0;

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                // here the magic
                let game_number = extract_game_number(&line_content);
                let counts = extract_counts(&line_content);

                let red_arr = counts
                    .iter()
                    .filter(|item| item.color == "red")
                    .find(|item| item.count > red);
                let green_arr = counts
                    .iter()
                    .filter(|item| item.color == "green")
                    .find(|item| item.count > green);
                let blue_arr = counts
                    .iter()
                    .filter(|item| item.color == "blue")
                    .find(|item| item.count > blue);

                if red_arr.is_some() || green_arr.is_some() || blue_arr.is_some() {
                    println!("Invalid game");
                } else {
                    sum += game_number;
                }
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
