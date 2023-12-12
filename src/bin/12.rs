use std::u32;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| (line.split_whitespace().nth(0).unwrap(),line.split_whitespace().nth(1).unwrap()))
        .collect::<Vec<(&str,&str)>>();
    let mut sum = 0;
    for line in map.iter() {
        sum += count_permutations(line.0, line.1);
    }
    Some(sum)
}

fn count_permutations(pattern: &str, key: &str) -> u32 {
    let key_vec = key.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let permutations: Vec<String> = generate_permutation_matches(pattern.to_owned(), &key_vec);

    permutations.len() as u32
}

fn generate_permutation_matches(pattern: String, key: &Vec<u32>) -> Vec<String> {
    let mut permutations = Vec::new();
    if !pattern.contains("?") {
        if permutation_matches(pattern.clone(), key) {
            permutations.push(pattern);
        }
    } else {
        if permutation_can_possibly_match(pattern.clone(), key) {
            permutations.extend(generate_permutation_matches(pattern.replacen("?", ".", 1), key));
            permutations.extend(generate_permutation_matches(pattern.replacen("?", "#", 1), key));
        }
    }

    permutations
}

fn permutation_can_possibly_match(permutation: String, key: &Vec<u32>) -> bool {
    let permutation_so_far = permutation.split("?").nth(0).unwrap().to_owned();
    let count = count_occurrences_in_permutation(permutation_so_far);

    if count.len() > key.len() {
        return false;
    }
    for (i, number) in count.into_iter().enumerate() {
        if key[i] < number {
            return false;
        }
    }

    true
}

fn permutation_matches(permutation: String, key: &Vec<u32>) -> bool {
    let count = count_occurrences_in_permutation(permutation);

    count == *key
}

fn count_occurrences_in_permutation(permutation: String) -> Vec<u32> {
    let mut count = Vec::new();
    let mut current_count = 0;
    for c in permutation.chars() {
        if c == '#' {
            current_count += 1;
        } else if c == '.' {
            if current_count > 0 {
                count.push(current_count);
                current_count = 0;
            }
        }
    }
    if current_count > 0 {
        count.push(current_count);
    }

    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| (line.split_whitespace().nth(0).unwrap(),line.split_whitespace().nth(1).unwrap()))
        .collect::<Vec<(&str,&str)>>();
    let mut sum = 0;
    for line in map.iter() {
        let pattern = format!("{}?{}?{}?{}?{}", line.0, line.0, line.0, line.0, line.0);
        let key = format!("{},{},{},{},{}", line.1, line.1, line.1, line.1, line.1);
        sum += count_permutations(pattern.as_str(), key.as_str());
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

#[test]
    fn test_permutation_can_possibly_match() {
        let test_cases = HashMap::from([
            (("###.##?????", vec![1, 2, 3, 4, 5]), false),
            (("###.##?????", vec![3, 2, 3, 4, 4]), true),
            (("###.##?????", vec![3, 4, 3, 3, 5]), true),
            (("###.##.....", vec![3, 2]), true),
            (("###.##.#", vec![3, 2]), false),
        ]);
        for (input, expected) in test_cases {
            assert_eq!(
                permutation_can_possibly_match(input.0.to_owned(), &input.1),
                expected
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
