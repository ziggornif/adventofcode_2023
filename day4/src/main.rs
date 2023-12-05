use regex::Regex;
use std::io::{BufRead, Error};
use utils::read_input_file;

fn extract_arrays(line: String) -> (Vec<i32>, Vec<i32>) {
    let re = Regex::new(r"Card\s*\d+: (.*?) \| (.*)").unwrap();
    let (result, winning) = if let Some(captures) = re.captures(&line) {
        let winning = captures[1]
            .split_whitespace()
            .flat_map(str::parse)
            .collect();
        let result = captures[2]
            .split_whitespace()
            .flat_map(str::parse)
            .collect();
        (winning, result)
    } else {
        (Vec::<i32>::new(), Vec::<i32>::new())
    };

    (result, winning)
}

fn process(input: String) -> Result<(i32, i32), Error> {
    let reader = read_input_file(input)?;
    let mut sum = 0;
    let mut cards = vec![0i32; reader.capacity()];

    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(line_content) => {
                cards[idx] += 1;
                let (winning, result) = extract_arrays(line_content);
                let matched = result
                    .iter()
                    .filter(|num| winning.iter().any(|win| &win == num));

                for v in 1..matched.clone().count() + 1 {
                    cards[idx + v] += cards[idx];
                }

                let score = matched.fold(0, |acc, _x| if acc == 0 { 1 } else { 2 * acc });
                sum += score;
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok((sum, cards.iter().sum()))
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 4 !");

    let (result, cards) = process("day4/src/resources/input.txt".to_owned())?;

    println!("The result is {} {}", result, cards);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn shoud_get_score() -> Result<(), String> {
        let (result, _) = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn shoud_get_score_real_input() -> Result<(), String> {
        let (result, _) = process("src/resources/input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 24160);
        Ok(())
    }

    #[test]
    fn shoud_get_total_cards() -> Result<(), String> {
        let (_, cards) = process("src/resources/test-input2.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(cards, 30);
        Ok(())
    }
}
