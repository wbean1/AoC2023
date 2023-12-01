use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut current_sum: u32 = 0;
    for line in lines {
        if line == "" { continue }
        let digits = extract_digits(&line);
        let line_num = digits[0] * 10 + digits[digits.len() - 1];
        current_sum = current_sum + line_num;
    }
    Some(current_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut current_sum: u32 = 0;
    for line in lines {
        if line == "" { continue }
        let new_line = replace_words(&line);
        let digits = extract_digits(&new_line);
        let line_num = digits[0] * 10 + digits[digits.len() - 1];
        current_sum = current_sum + line_num;
    }
    Some(current_sum)
}

fn replace_words(line: &str) -> String {
    let mut new_line = line.to_owned();
    // VERY TRICKY AND LAZY METHOD TO REPLACE
    // SINCE SOME LETTERS APPARENTLY CAN BE REUSED
    // IN OTHER NUMBER WORDS, WHICH MAKES THE ORDER
    // IN WHICH THEY ARE REPLACED MATTER, UNLESS YOU
    // DO THIS ONE LITTLE TRICK AND RETAIN THE LETTERS
    // TOO SO THEY CAN BE REUSED.  IM SO SMART.
    let replacements = HashMap::from([
        ("one", "on1e"),
        ("two", "tw2o"),
        ("three", "thr3ee"),
        ("four", "fo4ur"),
        ("five", "fi5ve"),
        ("six", "si6x"),
        ("seven", "se7ven"),
        ("eight", "eig8ht"),
        ("nine", "nin9e")
    ]);
    for (search, replace) in replacements {
        new_line = new_line.replace(search, replace);
    }
    new_line
}

fn extract_digits(line: &str) -> Vec<u32> {
    let mut vec = Vec::new();
    let digits = "123456789";
    for char in line.split_inclusive(|_| true) {
        if digits.contains(char) {
            vec.push(char.parse::<u32>().unwrap())
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
