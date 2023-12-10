use std::{collections::HashMap, io::Error};

use regex::Regex;

#[derive(Debug)]
struct Node {
    val: String,
    left: String,
    right: String,
}

fn least_common_multiple(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn parse(lines: Vec<String>) -> Result<(Vec<char>, Vec<Node>), Error> {
    let re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

    // here the directions
    let directions: Vec<char> = lines.get(0).unwrap().chars().collect();

    // and nodes
    let nodes: Vec<Node> = re
        .captures_iter(&lines.concat())
        .map(|caps| Node {
            val: caps[1].to_string(),
            left: caps[2].to_string(),
            right: caps[3].to_string(),
        })
        .collect();
    Ok((directions, nodes))
}

fn part1(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;

    let (directions, nodes) = parse(lines)?;

    let mut directions_iter = directions.iter().cycle();

    let mut curr_node = "AAA".to_string();
    let mut output = 0;

    while curr_node != "ZZZ" {
        let direction = *directions_iter.next().unwrap();
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
        output += 1;
    }
    println!("Output: {}", output);

    Ok(output)
}

fn part2(input: String) -> Result<usize, Error> {
    let lines = utils::read_input_file_as_vec(input)?;

    let (directions, nodes) = parse(lines)?;

    let starting_positions: Vec<String> = nodes
        .iter()
        .filter(|node| node.val.ends_with('A'))
        .map(|node| node.val.clone())
        .collect();

    let mut end_counts = HashMap::<String, usize>::new();

    for start in starting_positions.iter() {
        let mut current_node_name = start.clone();
        let mut count = 0;
        let mut directions_iter = directions.iter().cycle();

        while !current_node_name.ends_with('Z') {
            let current_instruction = directions_iter.next().unwrap();
            let current_node = nodes.iter().find(|n| n.val == current_node_name).unwrap();

            let next_node_name = if *current_instruction == 'R' {
                current_node.right.clone()
            } else {
                current_node.left.clone()
            };

            current_node_name = next_node_name;
            count += 1;
        }

        end_counts.insert(start.clone(), count);
    }

    let counts = end_counts.values().cloned().collect::<Vec<_>>();
    let lcm = least_common_multiple(&counts);

    Ok(lcm)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 8 !");

    let result = part1("day8/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result);

    let result_p2 = part2("day8/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result_p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn should_get_steps() -> Result<(), String> {
        let result = part1("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn should_manage_direction_repeat() -> Result<(), String> {
        let result = part1("src/resources/test-input2.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 6);
        Ok(())
    }

    #[test]
    fn should_manage_ghost_map() -> Result<(), String> {
        let result = part2("src/resources/test-input3.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 6);
        Ok(())
    }
}
