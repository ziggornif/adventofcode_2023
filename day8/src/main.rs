use std::io::Error;

use regex::Regex;

#[derive(Debug)]
struct Node {
    val: String,
    left: String,
    right: String,
}

fn process(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    // here the directions
    let directions: Vec<char> = lines.get(0).unwrap().chars().collect();

    // and nodes
    let nodes: Vec<Node> = re
        .captures_iter(&lines.concat())
        .filter_map(|caps| {
            Some(Node {
                val: caps[1].to_string(),
                left: caps[2].to_string(),
                right: caps[3].to_string(),
            })
        })
        .collect();

    let mut curr_node = "AAA".to_string();
    let mut idx = 0;
    let mut output = 0;

    while curr_node != "ZZZ" {
        let direction = directions[idx];
        if direction == 'R' {
            curr_node = nodes
                .iter()
                .find(|n| n.val == curr_node)
                .unwrap()
                .right
                .clone();
        } else {
            curr_node = nodes
                .iter()
                .find(|n| n.val == curr_node)
                .unwrap()
                .left
                .clone();
        }
        idx = (idx + 1) % directions.len();
        output += 1;
    }
    println!("Output: {}", output);

    Ok(output)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 8 !");

    let result = process("day8/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn should_get_steps() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn should_manage_direction_repeat() -> Result<(), String> {
        let result = process("src/resources/test-input2.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 6);
        Ok(())
    }
}
