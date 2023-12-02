use regex::Regex;

advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut current_sum: u32 = 0;
    for line in lines {
        if line == "" { continue }
        let (index, max_red, max_blue, max_green) = parse_line(line);
        if max_red > MAX_RED || max_blue > MAX_BLUE || max_green > MAX_GREEN { continue };
        current_sum = current_sum + index;
    }
    Some(current_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut current_sum: u32 = 0;
    for line in lines {
        if line == "" { continue }
        let (_, max_red, max_blue, max_green) = parse_line(line);
        current_sum = current_sum + (max_red * max_blue * max_green);
    }
    Some(current_sum)
}

fn parse_line(line: &str) -> (u32, u32, u32, u32) {
    let index_re = Regex::new(r"^Game (?<index>\d+):").unwrap();
    let Some(index_caps) = index_re.captures(line) else { panic!("no index found"); };
    let index = index_caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
    let red_re = Regex::new(r"(?<red>\d+) red").unwrap();
    let blue_re = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let green_re = Regex::new(r"(?<green>\d+) green").unwrap();
    let red_caps: Vec<u32> = red_re.captures_iter(line)
                                .map(|cap| cap.name("red").unwrap().as_str().parse::<u32>().unwrap())
                                .collect();
    let blue_caps: Vec<u32> = blue_re.captures_iter(line)
                                .map(|cap| cap.name("blue").unwrap().as_str().parse::<u32>().unwrap())
                                .collect();
    let green_caps: Vec<u32> = green_re.captures_iter(line)
                                .map(|cap| cap.name("green").unwrap().as_str().parse::<u32>().unwrap())
                                .collect();
    let max_red = red_caps.iter().max().unwrap_or(&0).to_owned();
    let max_blue = blue_caps.iter().max().unwrap_or(&0).to_owned();
    let max_green = green_caps.iter().max().unwrap_or(&0).to_owned();
    (index, max_red, max_blue, max_green)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 10 red, 2 blue, 3 green";
        let (index, max_red, max_blue, max_green) = parse_line(line);
        assert_eq!(index, 1);
        assert_eq!(max_red, 10);
        assert_eq!(max_blue, 2);
        assert_eq!(max_green, 3);
    }

    #[test]
    fn test_parse_line_no_red() {
        let line = "Game 1: 2 blue, 3 green";
        let (index, max_red, max_blue, max_green) = parse_line(line);
        assert_eq!(index, 1);
        assert_eq!(max_red, 0);
        assert_eq!(max_blue, 2);
        assert_eq!(max_green, 3);
    }

    #[test]
    fn test_parse_line_multiples() {
        let line = "Game 13: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let (index, max_red, max_blue, max_green) = parse_line(line);
        assert_eq!(index, 13);
        assert_eq!(max_red, 20);
        assert_eq!(max_blue, 6);
        assert_eq!(max_green, 13);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
