use std::collections::HashMap;
advent_of_code::solution!(11);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let expanded_map = expand_empty_rows(&expand_empty_cols(&map));

    let coordinates: Vec<Coord> = expanded_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == '#' {
                    Some(Coord {
                        x,
                        y,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    let mut distances: HashMap<(&Coord, &Coord), u32> = HashMap::new();
    for coord in coordinates.iter() {
        for other_coord in coordinates.iter() {
            if coord != other_coord && !distances.contains_key(&(coord, other_coord)) && !distances.contains_key(&(other_coord, coord)){
                let distance = ((coord.x as i32 - other_coord.x as i32).abs() + (coord.y as i32 - other_coord.y as i32).abs()) as u32;
                distances.insert((coord, other_coord), distance);
            }
        }
    }

    let mut sum = 0;
    for (_, value) in distances {
        sum += value;
    }

    Some(sum)
}

fn expand_empty_cols(map:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_map = map.clone();
    let col_len = expanded_map.len();
    for (i, _) in map[0].iter().enumerate().rev() {
        let mut is_blank = true;
        for (_, row) in map.iter().enumerate() {
            if row[i] != '.' {
                is_blank = false;
                break;
            }
        }
        if is_blank {
            for j in 0..col_len {
                expanded_map[j].insert(i, '.');
            }
        }
    }

    expanded_map
}

fn expand_empty_rows(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_map = map.clone();
    let row_len = expanded_map[0].len();
    let empty_row = vec!['.'; row_len];
    for (i, row) in map.iter().enumerate().rev() {
        let mut is_blank = true;
        for c in row.iter() {
            if *c != '.' {
                is_blank = false;
                break;
            }
        }
        if is_blank {
            expanded_map.insert(i, empty_row.clone());
        }
    }
    expanded_map
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let expanded_map = expand_arbitrary(&map);

    let coordinates: Vec<Coord> = expanded_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == '#' {
                    Some(Coord {
                        x,
                        y,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

        let mut distances: HashMap<(&Coord, &Coord), u64> = HashMap::new();
        for coord in coordinates.iter() {
            for other_coord in coordinates.iter() {
                if coord != other_coord && !distances.contains_key(&(coord, other_coord)) && !distances.contains_key(&(other_coord, coord)){
                    let mut distance = ((coord.x as i32 - other_coord.x as i32).abs() + (coord.y as i32 - other_coord.y as i32).abs()) as u64;
                    let expanded_rows_traversed = number_expanded_rows_between(coord.y, other_coord.y, &expanded_map);
                    let expanded_cols_traversed = number_expanded_cols_between(coord.x, other_coord.x, &expanded_map);
                    distance += (expanded_rows_traversed + expanded_cols_traversed) * 999999;
                    distances.insert((coord, other_coord), distance);
                }
            }
        }

        let mut sum = 0;
        for (_, value) in distances {
            sum += value;
        }

        Some(sum)
}

// use '|' to mark expanded rows and columns
// then we can just count the number of '|' between two points
fn expand_arbitrary(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_map = map.clone();

    // columns first:
    let col_len = expanded_map.len();
    for (i, _) in map[0].iter().enumerate().rev() {
        let mut is_blank = true;
        for (_, row) in map.iter().enumerate() {
            if row[i] == '#' {
                is_blank = false;
                break;
            }
        }
        if is_blank {
            for j in 0..col_len {
                expanded_map[j][i] = '|';
            }
        }
    }

    // rows next:
    let row_len = expanded_map[0].len();
    for (i, row) in map.iter().enumerate().rev() {
        let mut is_blank = true;
        for c in row.iter() {
            if *c == '#' {
                is_blank = false;
                break;
            }
        }
        if is_blank {
            for j in 0..row_len {
                expanded_map[i][j] = '|';
            }
        }
    }
    expanded_map
}

fn number_expanded_cols_between(x1: usize, x2: usize, map: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let mut start = x1;
    let mut end = x2;
    if x1 > x2 {
        start = x2;
        end = x1;
    }
    for i in start..end {
        if map[0][i] == '|' {
            count += 1;
        }
    }
    count
}

fn number_expanded_rows_between(y1: usize, y2: usize, map: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let mut start = y1;
    let mut end = y2;
    if y1 > y2 {
        start = y2;
        end = y1;
    }
    for i in start..end {
        if map[i][0] == '|' {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210)); // would have been nice for them to provide this...
    }
}
