use std::collections::HashMap;
advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_steps(input, 64)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_steps(input, 26501365)
}

fn part_one_steps(input: &str, steps: u32) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    map = add_padding(&map);

    let mut current_positions: HashMap<(usize, usize), bool> = get_current_positions(&map);

    for _ in 0..steps {
        current_positions = process_step(&map, &current_positions);
    }

    Some(current_positions.len() as u32)
}

// this was written entirely by co-pilot and ugly as sin
// but works so i'm not going to f with it
fn add_padding(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for y in 0..map.len() + 2 {
        let mut new_row: Vec<char> = Vec::new();
        for x in 0..map[0].len() + 2 {
            if y == 0 || y == map.len() + 1 || x == 0 || x == map[0].len() + 1 {
                new_row.push('#');
            } else {
                new_row.push(map[y - 1][x - 1]);
            }
        }
        new_map.push(new_row);
    }

    new_map
}

fn process_step(map: &Vec<Vec<char>>, current_positions: &HashMap<(usize, usize), bool>) -> HashMap<(usize, usize), bool> {
    let mut new_positions: HashMap<(usize, usize), bool> = HashMap::new();

    for (current_position, _) in current_positions.iter() {

        // I padded map so I can be lazy on bounds checking
        for position_to_check in [
            (current_position.0, current_position.1 - 1),
            (current_position.0 + 1, current_position.1),
            (current_position.0, current_position.1 + 1),
            (current_position.0 - 1, current_position.1),
        ].iter() {
            if map[position_to_check.1][position_to_check.0] == '.' || map[position_to_check.1][position_to_check.0] == 'S' {
                new_positions.insert(*position_to_check, true);
            }
        }
    }

    new_positions
}

fn process_step_infinite(map: &Vec<Vec<char>>, current_positions: &HashMap<(usize, usize, i32, i32), bool>) -> HashMap<(usize, usize, i32, i32), bool> {
    let mut new_positions: HashMap<(usize, usize,  i32, i32), bool> = HashMap::new();

    let map_y_len = map.len();
    let map_x_len = map[0].len();

    for (current_position, _) in current_positions.iter() {

        for position_to_check in [
            (current_position.0, (map_y_len + current_position.1 - 1) % map_y_len, current_position.2, if current_position.1 == 0 { current_position.3 - 1 } else { current_position.3 }),
            ((map_x_len + current_position.0 + 1) % map_x_len, current_position.1, if current_position.0 == map_x_len-1 { current_position.2 + 1 } else { current_position.2 }, current_position.3),
            (current_position.0, (map_y_len + current_position.1 + 1) % map_y_len, current_position.2, if current_position.1 == map_y_len-1 { current_position.3 + 1 } else { current_position.3 }),
            ((map_x_len + current_position.0 - 1) % map_x_len, current_position.1, if current_position.0 == 0 { current_position.2 - 1 } else { current_position.2 }, current_position.3),
        ].iter() {
            if map[position_to_check.1][position_to_check.0] == '.' || map[position_to_check.1][position_to_check.0] == 'S' {
                new_positions.insert(*position_to_check, true);
            }
        }
    }

    new_positions
}

fn get_current_positions(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), bool> {
    let mut current_positions: HashMap<(usize, usize), bool> = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                current_positions.insert((x, y), true);
            }
        }
    }

    current_positions
}

fn get_current_positions_infinite(map: &Vec<Vec<char>>) -> HashMap<(usize, usize, i32, i32), bool> {
    let mut current_positions: HashMap<(usize, usize, i32, i32), bool> = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                current_positions.insert((x, y, 0, 0), true);
            }
        }
    }

    current_positions
}

pub fn part_two_steps(input: &str, steps: u32) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut current_positions: HashMap<(usize, usize, i32, i32), bool> = get_current_positions_infinite(&map);

    for i in 0..steps {
        println!("step: {} of {}", i, steps);
        current_positions = process_step_infinite(&map, &current_positions);
    }

    Some(current_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_steps(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_6() {
        let result = part_two_steps(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_10() {
        let result = part_two_steps(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two_50() {
        let result = part_two_steps(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(1594));
    }

    #[test]
    fn test_part_two_100() {
        let result = part_two_steps(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(6536));
    }
}
