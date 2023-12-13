use std::u32;
use cached::proc_macro::cached;
use cached::{Cached, SizedCache};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| (line.split_whitespace().nth(0).unwrap(),line.split_whitespace().nth(1).unwrap()))
        .collect::<Vec<(&str,&str)>>();
    let mut sum = 0;
    for line in map.iter() {
        sum += count_permutations(&line.0.to_string(), &line.1.to_string());
    }
    Some(sum)
}

fn count_permutations(pattern: &String, key_str: &String) -> u64 {
    let key_vec = key_str.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    generate_permutation_matches(pattern, &key_vec)
}

#[cached(
    name = "GEN_PERM_MATCHES",
    type = "SizedCache<(String, Vec<u32>), u64>",
    create = "{ SizedCache::with_size(10000000) }",
)]
fn generate_permutation_matches(pattern: &String, key_vec: &Vec<u32>) -> u64 {
    let mut count = 0;
    if !pattern.contains("?") {
        if permutation_matches(pattern, key_vec) {
            count += 1;
        }
    } else {
        if permutation_can_possibly_match(pattern, key_vec) {
            // ok so the trick here is to replace '..' with '.' so that we get better cache usage.
            // in terms of matching the key, '..' and '.' accomplish the same thing.
            // without this, test_part_two takes ~14min to run
            // with this, it takes 5 seconds.
            count += generate_permutation_matches(&pattern.replacen("?", ".", 1).replace("..", "."), key_vec);
            count += generate_permutation_matches(&pattern.replacen("?", "#", 1).replace("..", "."), key_vec);
        }
    }

    count
}

// caching here does not help
fn permutation_can_possibly_match(permutation: &String, key_vec: &Vec<u32>) -> bool {
    let permutation_so_far = permutation.split("?").nth(0).unwrap().to_owned();
    let count = count_occurrences_in_permutation(&permutation_so_far);

    if count.len() == 0 {
        return true;
    }

    if count.len() > key_vec.len() {
        return false;
    }
    // the last value doesn't need to be exact, just less than or equal, so we skip it
    for (i, number) in count.iter().rev().skip(1).enumerate() {
        if key_vec[count.len()-2-i] != *number {
            return false;
        }
    }
    let last_index = count.len()-1;
    if count[last_index] > key_vec[last_index] {
        return false;
    }

    true
}

// caching here does not help
fn permutation_matches(permutation: &String, key_vec: &Vec<u32>) -> bool {
    let count = count_occurrences_in_permutation(permutation);

    count == *key_vec
}

#[cached(
    name = "COUNT_OCCUR",
    type = "SizedCache<String, Vec<u32>>",
    create = "{ SizedCache::with_size(10000000) }",
)]
fn count_occurrences_in_permutation(permutation: &String) -> Vec<u32> {
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

pub fn part_two(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| (line.split_whitespace().nth(0).unwrap(),line.split_whitespace().nth(1).unwrap()))
        .collect::<Vec<(&str,&str)>>();
    let mut sum = 0;
    for line in map.iter() {
        let pattern = format!("{}?{}?{}?{}?{}", line.0, line.0, line.0, line.0, line.0);
        let key = format!("{},{},{},{},{}", line.1, line.1, line.1, line.1, line.1);
        let result = count_permutations(&pattern, &key);
        sum += result;
        let mut cache1 = GEN_PERM_MATCHES.lock().unwrap();
        let mut cache2 = COUNT_OCCUR.lock().unwrap();
        // println!("GEN_PERM_MATCHES: hits: {}, misses: {}", cache1.cache_hits().unwrap_or(0), cache1.cache_misses().unwrap_or(0));
        // println!("COUNT_OCCUR: hits: {}, misses: {}", cache2.cache_hits().unwrap_or(0), cache2.cache_misses().unwrap_or(0));
        cache1.cache_clear();
        cache1.cache_reset_metrics();
        cache2.cache_clear();
        cache2.cache_reset_metrics();
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
            (("###.##?????", vec![1, 2, 3]), false),
            (("###.##?????", vec![3, 2, 3]), true),
            (("###.##?????", vec![3, 4, 3]), true),
            (("###.##.....", vec![3, 2]), true),
            (("###.##.?...", vec![3, 2]), true),
            (("###.##.#", vec![3, 2]), false),
        ]);
        for (input, expected) in test_cases {
            assert_eq!(
                permutation_can_possibly_match(&input.0.to_string(), &input.1),
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
