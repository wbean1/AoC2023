use std::collections::HashMap;
use num::integer::lcm;
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

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = parse_map(input);
    let nodes = find_nodes_ending_with(&map, 'A');
    let mut cycles = Vec::new();
    // i really don't think this is safe to assume, but whatever.  it works for the input
    for mut node in nodes {
        let mut number_steps: u64 = 0;
        let mut current_instruction = instructions.to_string();
        while node.chars().nth(2).unwrap() != 'Z' {
            let (left, right) = map.get(node.as_str()).unwrap().to_owned();
            if current_instruction.chars().nth(0).unwrap() == 'L' {
                node = left.to_string();
                current_instruction.remove(0);
                current_instruction.push('L');
            } else {
                node = right.to_string();
                current_instruction.remove(0);
                current_instruction.push('R');
            }
            number_steps += 1;
        }
        cycles.push(number_steps);
    }
    Some(lcm_vec(&cycles))
}

fn lcm_vec(v: &Vec<u64>) -> u64 {
    let mut result = v[0];
    for i in 1..v.len() {
        result = lcm(result, v[i]);
    }
    result
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
