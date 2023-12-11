use std::io::Error;

#[derive(Debug)]
struct Pos(i64, i64);

fn duplicate_lines_and_columns(map: &mut Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    // Duplicate empty lines
    for row in (0..rows).rev() {
        if map[row].iter().all(|&c| c == '.') {
            map.insert(row + 1, map[row].clone());
        }
    }

    // Duplicate empty columns
    for col in (0..cols).rev() {
        if map.iter().all(|row| row[col] == '.') {
            for row in map.iter_mut() {
                row.insert(col + 1, row[col]);
            }
        }
    }
}

fn distance(a: &Pos, b: &Pos) -> i64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dx.abs() + dy.abs()
}

fn part1(input: String) -> Result<i64, Error> {
    let lines = utils::read_input_file_as_vec(input)?;

    let mut universe_map: Vec<Vec<char>> =
        lines.iter().map(|line| line.chars().collect()).collect();

    duplicate_lines_and_columns(&mut universe_map);

    let galaxies: Vec<Pos> = universe_map
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, &c)| {
                if c == '#' {
                    Some(Pos(row as i64, col as i64))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut total_shortest_paths = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            total_shortest_paths += distance(&galaxies[i], &galaxies[j]);
        }
    }

    Ok(total_shortest_paths)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 10 !");
    let result = part1("day11/src/resources/input.txt".to_string())?;
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn should_get_p1_result() -> Result<(), String> {
        let result = part1("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 374);
        Ok(())
    }
}
