use core::u64::MAX;

advent_of_code::solution!(5);

const DIGITS: &str = "0123456789";

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split("\n");
    let first_line = lines.clone().nth(0).unwrap().split(": ").nth(1).unwrap();
    let seeds: Vec<u64> = first_line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();

    let sections: Vec<Vec<(u64,u64,u64)>> = parse_sections(lines.clone().skip(2).collect());
    let least_location = plant_seeds(&seeds, &sections);

    Some(least_location)
}

fn plant_seeds(seeds: &Vec<u64>, sections: &Vec<Vec<(u64,u64,u64)>>) -> u64 {
    let mut least_location: u64 = MAX;
    for seed in seeds {
        let mut current_value = *seed;
        for section in sections.iter() {
            // destination, current, range
            for line in section.iter() {
                if line.1 <= current_value && line.1 + line.2 > current_value {
                    current_value = line.0 + (current_value - line.1);
                    break;
                }
            }
        }
        if current_value < least_location {
            least_location = current_value;
        }
    }

    least_location
}

fn parse_sections(lines: Vec<&str>) -> Vec<Vec<(u64, u64, u64)>> {
    let mut sections: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut section: Vec<(u64, u64, u64)> = Vec::new();
    for line in lines {
        if line == "" {
            sections.push(section);
            section = Vec::new();
        } else if !DIGITS.contains(line.chars().nth(0).unwrap()) {
             continue
        } else {
            section.push(parse_line(line));
        }
    }
    sections.push(section);
    sections
}

fn parse_line(line: &str) -> (u64, u64, u64) {
    let mut parts = line.split(" ");
    let destination = parts.next().unwrap().parse::<u64>().unwrap();
    let current = parts.next().unwrap().parse::<u64>().unwrap();
    let range = parts.next().unwrap().parse::<u64>().unwrap();

    (destination, current, range)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split("\n");
    let first_line = lines.clone().nth(0).unwrap().split(": ").nth(1).unwrap();
    let seeds_initial: Vec<u64> = first_line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    let seeds: Vec<u64> = expand_seeds(&seeds_initial);

    let sections: Vec<Vec<(u64,u64,u64)>> = parse_sections(lines.clone().skip(2).collect());

    let least_location = plant_seeds(&seeds, &sections);

    Some(least_location)
}

fn expand_seeds(seeds: &Vec<u64>) -> Vec<u64> {
    let mut expanded_seeds: Vec<u64> = Vec::new();
    for pair in seeds.chunks(2) {
        for n in 0..pair[1] {
            expanded_seeds.push(pair[0] + n);
        }
    }
    expanded_seeds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
