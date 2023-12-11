use rayon::prelude::*;
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

fn expand_universe(map: &mut Vec<Vec<char>>, expansion_factor: usize) {
    let rows = map.len();
    let cols = map[0].len();

    // Duplicate empty rows
    for row in (0..rows).rev() {
        if map[row].iter().all(|&c| c == '.') {
            let empty_row = vec!['.'; cols];
            if expansion_factor == 1 {
                map.insert(row + 1, empty_row.clone());
            } else {
                for _ in 0..expansion_factor - 1 {
                    map.insert(row + 1, empty_row.clone());
                }
            }
        }
    }

    // Duplicate empty columns
    for col in (0..cols).rev() {
        if map.iter().all(|row| row[col] == '.') {
            if expansion_factor == 1 {
                for row in map.iter_mut() {
                    row.insert(col + 1, row[col]);
                }
            } else {
                for _ in 0..expansion_factor - 1 {
                    for row in map.iter_mut() {
                        row.insert(col + 1, row[col]);
                    }
                }
            }
        }
    }
}

fn distance(a: &Pos, b: &Pos) -> i64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dx.abs() + dy.abs()
}

// fn part1(input: String) -> Result<i64, Error> {
//     let lines = utils::read_input_file_as_vec(input)?;

//     let mut universe_map: Vec<Vec<char>> =
//         lines.iter().map(|line| line.chars().collect()).collect();

//     duplicate_lines_and_columns(&mut universe_map);

//     let galaxies: Vec<Pos> = universe_map
//         .iter()
//         .enumerate()
//         .flat_map(|(row, line)| {
//             line.iter().enumerate().filter_map(move |(col, &c)| {
//                 if c == '#' {
//                     Some(Pos(row as i64, col as i64))
//                 } else {
//                     None
//                 }
//             })
//         })
//         .collect();

//     let mut total_shortest_paths = 0;

//     for i in 0..galaxies.len() {
//         for j in (i + 1)..galaxies.len() {
//             total_shortest_paths += distance(&galaxies[i], &galaxies[j]);
//         }
//     }

//     Ok(total_shortest_paths)
// }

fn calculate_total_shortest_paths(galaxies: &[Pos]) -> i64 {
    galaxies
        .par_iter()
        .enumerate()
        .flat_map(|(i, pos_i)| {
            (i + 1..galaxies.len())
                .into_par_iter()
                .map(move |j| distance(pos_i, &galaxies[j]))
        })
        .sum()
}

fn process(input: String, factor: usize) -> Result<i64, Error> {
    let lines = utils::read_input_file_as_vec(input)?;

    let mut universe_map: Vec<Vec<char>> =
        lines.iter().map(|line| line.chars().collect()).collect();

    expand_universe(&mut universe_map, factor);

    let galaxies: Vec<Pos> = universe_map
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

    // let mut total_shortest_paths = 0;

    // for i in 0..galaxies.len() {
    //     for j in (i + 1)..galaxies.len() {
    //         total_shortest_paths += distance(&galaxies[i], &galaxies[j]);
    //     }
    // }
    let total_shortest_paths = calculate_total_shortest_paths(&galaxies);

    Ok(total_shortest_paths)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 10 !");
    let result = process("day11/src/resources/input.txt".to_string(), 1)?;
    println!("Result: {}", result);

    let result = process("day11/src/resources/input.txt".to_string(), 1000000)?;
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
        let result = process("src/resources/test-input.txt".to_owned(), 10)
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 1030);
        Ok(())
    }

    #[test]
    fn should_get_p2_100_result() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned(), 100)
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 8410);
        Ok(())
    }
}
