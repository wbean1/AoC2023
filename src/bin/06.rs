advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let time_distance_map = parse_input(input);
    let mut total_ways_to_win = 1;
    for (time, distance) in time_distance_map {
        let mut ways_to_win = 0;
        // i is the amount of time to hold, so also our velocity
        for i in 1..time-1 {
            if i * (time - i) > distance {
                ways_to_win += 1;
            }
        }
        total_ways_to_win *= ways_to_win;
    }
    Some(total_ways_to_win)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, distance) = parse_input_part_two(input);
    let mut ways_to_win = 0;
    for i in 1..time-1 {
        if i * (time - i) > distance {
            ways_to_win += 1;
        }
    }
    Some(ways_to_win)
}

fn parse_input_part_two(input: &str) -> (u64, u64) {
    let mut time_line = input.lines().next().unwrap();
    let mut distance_line = input.lines().nth(1).unwrap();
    time_line = time_line.split(": ").nth(1).unwrap();
    distance_line = distance_line.split(": ").nth(1).unwrap();
    let time: u64 = remove_whitespace(time_line).parse().unwrap();
    let distance: u64 = remove_whitespace(distance_line).parse().unwrap();
    (time, distance)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let mut races = Vec::new();
    let mut time_line = input.lines().next().unwrap();
    let mut distance_line = input.lines().nth(1).unwrap();
    time_line = time_line.split(": ").nth(1).unwrap();
    distance_line = distance_line.split(": ").nth(1).unwrap();
    let times: Vec<u32> = time_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<u32> = distance_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for (i, time) in times.into_iter().enumerate() {
        races.push((time, distances[i]));
    };
    races
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
