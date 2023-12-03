use std::io::{BufRead, Error};
use utils::read_input_file;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Number {
    start: Coord,
    end: Coord,
    value: i32,
}

#[derive(Debug)]
struct Symbol {
    coord: Coord,
}

fn extract_numbers_adjacent_to_symbols(input: String) -> Result<i32, Error> {
    let reader = read_input_file(input)?;

    let mut numbers = Vec::<Number>::new();
    let mut symbols = Vec::<Symbol>::new();

    for (row_idx, line) in reader.lines().enumerate() {
        let mut start_coord = 0;
        let mut value = String::new();

        let line = line?;
        let line_size = line.chars().count();

        for (col_idx, ch) in line.chars().enumerate() {
            if ch != '.' && !ch.is_numeric() {
                if value.len() > 0 {
                    numbers.push(Number {
                        start: Coord {
                            x: row_idx as i32,
                            y: start_coord,
                        },
                        end: Coord {
                            x: row_idx as i32,
                            y: (col_idx - 1) as i32,
                        },
                        value: value.parse::<i32>().unwrap(),
                    });
                    value.clear();
                }

                symbols.push(Symbol {
                    coord: Coord {
                        x: row_idx as i32,
                        y: col_idx as i32,
                    },
                })
            } else if ch.is_numeric() {
                if value.len() == 0 {
                    start_coord = col_idx as i32;
                } else if col_idx == line_size - 1 {
                    value.push(ch);
                    numbers.push(Number {
                        start: Coord {
                            x: row_idx as i32,
                            y: start_coord,
                        },
                        end: Coord {
                            x: row_idx as i32,
                            y: col_idx as i32,
                        },
                        value: value.parse::<i32>().unwrap(),
                    });
                    value.clear();
                }
                value.push(ch);
            } else {
                if value.len() > 0 {
                    numbers.push(Number {
                        start: Coord {
                            x: row_idx as i32,
                            y: start_coord,
                        },
                        end: Coord {
                            x: row_idx as i32,
                            y: (col_idx - 1) as i32,
                        },
                        value: value.parse::<i32>().unwrap(),
                    });
                    value.clear();
                }
            }
        }
    }

    let mut sum = 0;
    for number in numbers {
        let found = symbols.iter().find(|symb| {
            let s_x = symb.coord.x;
            let s_y = symb.coord.y;

            let matching = if number.start.x != s_x
                && number.start.x != s_x + 1
                && number.start.x != s_x - 1
            {
                false
            } else {
                if number.start.y - 1 <= s_y && s_y <= number.end.y + 1 {
                    true
                } else {
                    false
                }
            };
            matching
        });

        if found.is_some() {
            sum += number.value;
        }
    }

    Ok(sum)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 3 !");

    let result = extract_numbers_adjacent_to_symbols("day3/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extract_numbers_adjacent_to_symbols;

    #[test]
    fn should_do_something() -> Result<(), String> {
        let result = extract_numbers_adjacent_to_symbols("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 4361);
        Ok(())
    }
}
