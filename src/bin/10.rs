use std::collections::HashMap;
advent_of_code::solution!(10);

const CONNECTED_SOUTH: &str = "S7F|";
const CONNECTED_NORTH: &str = "SJL|";
const CONNECTED_EAST: &str = "SFL-";
const CONNECTED_WEST: &str = "SJ7-";

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let steps = count_steps_in_loop(&map);

    Some(steps)
}

fn count_steps_in_loop(map: &Vec<Vec<char>>) -> u32 {
    let mut current_position = find_starting_point(&map);
    let mut replaced_map = map.clone();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

    while !visited.contains_key(&current_position) {
        visited.insert(current_position, true);
        let (i,j) = current_position;

        let next_position = {
            if i > 0 && CONNECTED_NORTH.contains(replaced_map[i][j]) && CONNECTED_SOUTH.contains(replaced_map[i-1][j]) { (i-1, j) }
            else if j < replaced_map[i].len() - 1 && CONNECTED_EAST.contains(replaced_map[i][j]) && CONNECTED_WEST.contains(replaced_map[i][j+1]) { (i, j+1) }
            else if i < replaced_map.len() - 1 && CONNECTED_SOUTH.contains(replaced_map[i][j]) && CONNECTED_NORTH.contains(replaced_map[i+1][j]) { (i+1, j) }
            else if j > 0 && CONNECTED_WEST.contains(replaced_map[i][j]) && CONNECTED_EAST.contains(replaced_map[i][j-1]) { (i, j-1) }
            else { panic!("No valid next step from {:?}", current_position); }
        };
        if replaced_map[i][j] != 'S' { replaced_map[i][j] = '.' } else { replaced_map[i][j] = '|' }; // this is a hack
        current_position = next_position;
    }
    visited.keys().count() as u32 / 2
}

fn find_starting_point(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_starting_point() {
        let test_map: Vec<Vec<char>> = vec![
            vec!['.', '|', '7', 'F', '|'],
            vec!['-', 'J', 'L', 'S', '|'],
            vec!['.', '-', '-', '-', '|'],
        ];
        let result = find_starting_point(&test_map);
        assert_eq!(result, (1, 3));
    }

    #[test]
    fn test_count_connected() {
        let test_map: Vec<Vec<char>> = vec![
            vec!['.', '|', '7', 'F', '|'],
            vec!['-', 'J', 'L', 'S', '|'],
            vec!['.', '-', '-', '-', '|'],
        ];
        let tests: HashMap<(usize, usize), u32> = HashMap::from([
            ((0,0), 0),
            ((0,1), 1),
            ((0,2), 1),
            ((0,3), 1),
            ((0,4), 1),
            ((1,0), 1),
            ((1,1), 2),
            ((1,2), 2),
            ((1,3), 2),
            ((1,4), 2),
            ((2,0), 0),
            ((2,1), 1),
            ((2,2), 2),
            ((2,3), 1),
            ((2,4), 1),
        ]);
        for (key, value) in tests {
            let (i, j) = key;
            let result = count_connected(&test_map, i, j);
            assert_eq!(result, value, "Failed for {:?}", key);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_more_complex() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_even_more_complex() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
