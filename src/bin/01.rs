use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut current_sum: u32 = 0;
    for line in lines {
        if line == "" { continue }
        let digits = extract_digits(&line);
        current_sum = current_sum + score_digits(digits);
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
        current_sum = current_sum + score_digits(digits);
    }
    Some(current_sum)
}

fn score_digits(digits: Vec<u32>) -> u32 {
    digits[0] * 10 + digits[digits.len() - 1]
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
    fn test_extract_digits() {
        let result = extract_digits("a1b2c3d45sdfs6ogi7weq890");
        assert_eq!(result, vec![1,2,3,4,5,6,7,8,9])
    }

    #[test]
    fn test_replace_words() {
        let result = extract_digits(replace_words("9abonecdefgeightwoeighthree9").as_str());
        assert_eq!(result, vec![9,1,8,2,8,3,9])
    }

    #[test]
    fn test_score_digits() {
        let cases = HashMap::from([
            (vec![1,2], 12),
            (vec![1,2,3], 13),
            (vec![9,2,3,4,2,3,4,2,1], 91),
        ]);
        for (digits, result) in cases {
            assert_eq!(result, score_digits(digits))
        }
    }

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
