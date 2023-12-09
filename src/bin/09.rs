advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let number_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let total: i32 = number_lines
        .into_iter()
        .map(|line| find_next_value(line))
        .sum();
    Some(total)
}

fn find_next_value(line: Vec<i32>) -> i32 {
    match all_zeroes(&line) {
        true => 0,
        false => {
            let difference_line = line
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, number)| *number - line[i])
                .collect::<Vec<i32>>();
            find_next_value(difference_line) + line.last().unwrap()
        }
    }
}

fn all_zeroes(line: &Vec<i32>) -> bool {
    line.iter().all(|&x| x == 0)
}

pub fn part_two(input: &str) -> Option<i32> {
    let number_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let total: i32 = number_lines
        .into_iter()
        .map(|line| find_previous_value(line))
        .sum();

    Some(total)
}

fn find_previous_value(line: Vec<i32>) -> i32 {
    match all_zeroes(&line) {
        true => 0,
        false => {
            let difference_line = line
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, number)| *number - line[i])
                .collect::<Vec<i32>>();
              line.first().unwrap() - find_previous_value(difference_line)
        }
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
