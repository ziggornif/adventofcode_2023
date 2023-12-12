use rayon::prelude::*;
use std::io::Error;

#[derive(Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
struct Pos(i64, i64);

fn calculate_empty_lig_cols(universe_map: &Vec<Vec<char>>) -> (Vec<i64>, Vec<i64>) {
    let empty_lines = universe_map
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|&c| c == '.'))
        .map(|(id, _)| id as i64)
        .collect::<Vec<i64>>();

    let mut empty_cols = Vec::<i64>::new();
    let cols = universe_map[0].len();
    for col in (0..cols).rev() {
        if universe_map.iter().all(|row| row[col] == '.') {
            empty_cols.push(col as i64)
        }
    }

    empty_cols.sort();

    (empty_lines, empty_cols)
}
fn distance(a: Pos, b: Pos, factor: i64, empty_lines: &Vec<i64>, empty_cols: &Vec<i64>) -> i64 {
    let (startx, endx) = if b.0 > a.0 { (a.0, b.0) } else { (b.0, a.0) };

    let num_lines = empty_lines
        .iter()
        .filter(|&&l| l > startx && l < endx)
        .count() as i64;

    let (starty, endy) = if b.1 > a.1 { (a.1, b.1) } else { (b.1, a.1) };

    let num_cols = empty_cols
        .iter()
        .filter(|&&c| c > starty && c < endy)
        .count() as i64;

    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    dx.abs() + dy.abs() + (num_lines + num_cols) * factor
}

fn process(input: String, factor: i64) -> Result<i64, Error> {
    let lines = utils::read_input_file_as_vec(input)?;

    let universe_map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut galaxies: Vec<Pos> = universe_map
        .par_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.par_iter().enumerate().filter_map(move |(col, &c)| {
                if c == '#' {
                    Some(Pos(row as i64, col as i64))
                } else {
                    None
                }
            })
        })
        .collect();

    galaxies.sort_by(|a, b| a.cmp(b));

    let mut total_shortest_paths = 0;

    let (empty_lines, empty_cols) = calculate_empty_lig_cols(&universe_map);

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            total_shortest_paths += distance(
                galaxies[i].clone(),
                galaxies[j].clone(),
                factor,
                &empty_lines,
                &empty_cols,
            );
        }
    }

    Ok(total_shortest_paths)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 10 !");
    let result = process("day11/src/resources/input.txt".to_string(), 1)?;
    println!("Result: {}", result);

    let result = process("day11/src/resources/input.txt".to_string(), 999999)?;
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn should_get_p1_result() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned(), 1)
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 374);
        Ok(())
    }

    #[test]
    fn should_get_p2_10_result() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned(), 9)
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 1030);
        Ok(())
    }

    #[test]
    fn should_get_p2_100_result() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned(), 99)
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 8410);
        Ok(())
    }
}
