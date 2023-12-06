use std::io::{BufRead, Error};

use utils::read_input_file;

fn find_lowest_winning_opts(race_time: i64, best_distance: i64) -> i64 {
    let mut time_to_hold = 0;
    for n in 0..race_time {
        if ((race_time - n) * n) > best_distance {
            time_to_hold += 1;
        }
    }
    time_to_hold
}

fn part1(input: String) -> Result<i64, Error> {
    let reader = read_input_file(input)?;

    let input_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let race_lengths: Vec<i64> = input_lines[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let best_distances: Vec<i64> = input_lines[1]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut total_score = 1;
    for i in 0..race_lengths.len() {
        total_score *= find_lowest_winning_opts(race_lengths[i], best_distances[i]);
    }

    Ok(total_score)
}

fn part2(input: String) -> Result<i64, Error> {
    let reader = read_input_file(input)?;

    let input_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let race_lengths: String = input_lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse::<u32>().ok())
        .map(|n| n.to_string())
        .collect();

    let best_distances: String = input_lines[1]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse::<u32>().ok())
        .map(|n| n.to_string())
        .collect();

    let race_length: i64 = race_lengths.parse().unwrap_or(0);
    let best_distance: i64 = best_distances.parse().unwrap_or(0);

    println!("{} {}", race_length, best_distance);
    let total_score = find_lowest_winning_opts(race_length, best_distance);
    Ok(total_score)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 6 !");

    let result_p1 = part1("day6/src/resources/input.txt".to_owned())?;
    let result_p2 = part2("day6/src/resources/input.txt".to_owned())?;

    println!("The result is p1 {} p2 {}", result_p1, result_p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn shoud_get_score() -> Result<(), String> {
        let result = part1("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 288);
        Ok(())
    }

    #[test]
    fn shoud_get_score_p2() -> Result<(), String> {
        let result = part2("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        // assert_eq!(result, 288);
        println!("{result}");
        Ok(())
    }
}
