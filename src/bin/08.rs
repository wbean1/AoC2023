use std::collections::HashMap;
use regex::Regex;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, map) = parse_map(input);
    let mut current_instruction = instructions.to_string();
    let mut current_node = "AAA";
    let mut number_steps = 0;
    while current_node != "ZZZ" {
        let (left, right) = map.get(current_node).unwrap().to_owned();
        if current_instruction.chars().nth(0).unwrap() == 'L' {
            current_node = left;
            current_instruction.remove(0);
            current_instruction.push('L');

        } else {
            current_node = right;
            current_instruction.remove(0);
            current_instruction.push('R');
        }
        number_steps += 1;
    }

    Some(number_steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (instructions, map) = parse_map(input);
    let mut current_instruction = instructions.to_string();
    let mut current_nodes = find_nodes_ending_with(&map, 'A');
    let mut number_steps = 0;
    while !nodes_end_with(&current_nodes, 'Z') {
        // figure out which direction we're moving and adjust the instruction string
        let move_left = current_instruction.chars().nth(0).unwrap() == 'L';
        current_instruction.remove(0);
        if move_left {
            current_instruction.push('L');
        } else {
            current_instruction.push('R');
        }

        // move to the next nodes
        let mut destination_nodes: Vec<String> = Vec::new();
        for node in current_nodes {
            let destination_pair = map.get(node.as_str()).unwrap();
            if move_left {
                destination_nodes.push(destination_pair.0.to_string());
            } else {
                destination_nodes.push(destination_pair.1.to_string());
            }
        }
        current_nodes = destination_nodes;

        // increment steps taken
        number_steps += 1;
    }

    Some(number_steps)
}

fn nodes_end_with(nodes: &Vec<String>, c: char) -> bool {
    for node in nodes {
        if node.chars().nth(2).unwrap() != c {
            return false
        }
    }

    true
}

fn find_nodes_ending_with(map: &HashMap<&str, (&str, &str)>, c: char) -> Vec<String> {
    let mut matched_keys = Vec::new();
    for node in map.keys().cloned() {
        if node.chars().nth(2).unwrap() == c {
            matched_keys.push(node.to_string());
        }
    }

    matched_keys
}

fn parse_map(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let instruction_str = input.lines().nth(0).unwrap();
    let mut map: HashMap<&str, (&str,&str)> = HashMap::new();
    let re = Regex::new(r"^(?<first>\w+) = \((?<second>\w+), (?<third>\w+)\)").unwrap();
    for line in input.lines().skip(2) {
        let Some(caps) = re.captures(line) else { continue };
        map.insert(caps.name("first").unwrap().as_str(),
                  (caps.name("second").unwrap().as_str(), caps.name("third").unwrap().as_str()));
    }
    (instruction_str, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
