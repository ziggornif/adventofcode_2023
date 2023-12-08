// RL

// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)

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
    println!("directions {:?}", directions);

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

    let mut exit = false;
    let mut dir_id = 0;
    let mut curr_node = nodes.get(0).unwrap();
    let mut steps = 0;
    while !exit {
        steps += 1;
        if dir_id == directions.len() {
            dir_id = 0
        }
        let direction = *directions.get(dir_id).unwrap();
        curr_node = nodes
            .iter()
            .find(|node| {
                if direction == 'L' {
                    node.val == curr_node.left
                } else {
                    node.val == curr_node.right
                }
            })
            .unwrap();

        println!("steps {steps} direction {direction} node {:?}", curr_node);
        if curr_node.val == "ZZZ" {
            exit = false;
        }
    }
    Ok(steps)
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
