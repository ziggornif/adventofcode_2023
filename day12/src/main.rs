use std::io::Error;

#[derive(Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Line {
    springs: Vec<Spring>,
    damaged_groups: Vec<i32>,
}

fn process(input: String) -> Result<i32, Error> {
    let lines: Vec<String> = utils::read_input_file_as_vec(input)?;
    let mut spring_lines = Vec::<Line>::new();
    lines.iter().for_each(|line| {
        let mut slices = line.split_whitespace();
        let springs = slices
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                _ => Spring::Unknown,
            })
            .collect();
        let damaged_groups = slices
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        spring_lines.push(Line {
            springs,
            damaged_groups,
        })
    });

    Ok(0)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 12 !");
    let result = process("day12/src/resources/input.txt".to_owned())?;
    println!("Result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn should_get_p1_result() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 21);
        Ok(())
    }
}
