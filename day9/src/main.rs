use std::io::Error;

fn find_prev_value(values: &Vec<i32>) -> i32 {
    let mut reversed = values.clone();
    reversed.reverse();

    find_next_value(&reversed)
}

fn find_next_value(values: &Vec<i32>) -> i32 {
    let differences = values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let last = values.last().unwrap();

    if differences.iter().all(|&d| d == 0) {
        return *last;
    }

    let next_difference = find_next_value(&differences);
    *last + next_difference
}

fn part1(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;
    let values = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut result = 0;
    values
        .iter()
        .for_each(|line| result += find_next_value(line));
    println!("{:?}", result);
    Ok(result)
}

fn part2(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;
    let values = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut result = 0;
    values
        .iter()
        .for_each(|line| result += find_prev_value(line));
    println!("{:?}", result);
    Ok(result)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 9 !");

    let result = part1("day9/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result);

    let result_p2 = part2("day9/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result_p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn should_get_p1_result() -> Result<(), String> {
        let result = part1("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 114);
        Ok(())
    }

    #[test]
    fn should_get_p2_result() -> Result<(), String> {
        let result = part2("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 2);
        Ok(())
    }
}
