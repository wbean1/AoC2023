advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let number_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let mut total = 0;
    for line in number_lines.into_iter() {
        total += find_next_value(line);
    }
    Some(total)
}

fn find_next_value(line: Vec<i32>) -> i32 {
    if all_zeroes(&line) {
        return 0;
    } else {
        let mut difference_line = Vec::new();
        for (i, number) in line.iter().skip(1).enumerate() {
            difference_line.push(*number - line[i]);
        }
        return find_next_value(difference_line) + line.last().unwrap();
    }
}

fn all_zeroes(line: &Vec<i32>) -> bool {
    for number in line.iter() {
        if *number != 0 {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<i32> {
    let number_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let mut total = 0;
    for line in number_lines.into_iter() {
        total += find_previous_value(line);
    }
    Some(total)
}

fn find_previous_value(line: Vec<i32>) -> i32 {
    if all_zeroes(&line) {
        return 0;
    } else {
        let mut difference_line = Vec::new();
        for (i, number) in line.iter().skip(1).enumerate() {
            difference_line.push(*number - line[i]);
        }
        return line.first().unwrap() - find_previous_value(difference_line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
