advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let maps = parse_input(input);
    let mut sum = 0;
    for map in maps {
        sum += 1 * find_vertical_mirror(&map);
        sum += 100 * find_horizontal_mirror(&map);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_vertical_mirror(map: &Vec<Vec<char>>) -> u32 {
    let transposed_map = transpose_map(map);

    find_horizontal_mirror(&transposed_map)
}

fn transpose_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed_map = Vec::new();
    for i in 0..map[0].len() {
        let mut row = Vec::new();
        for j in 0..map.len() {
            row.push(map[j][i]);
        }
        transposed_map.push(row);
    }

    transposed_map
}

fn find_horizontal_mirror(map: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;

    for (i, row) in map.iter().enumerate().skip(1) {
        if check_horizontal_mirror_at(&map, i) {
            count = i as u32;
            break;
        }
    }

    count
}

fn check_horizontal_mirror_at(map: &Vec<Vec<char>>, i: usize) -> bool {
    if i >= map.len() {
        return false;
    }
    let top = map.split_at(i).0.to_vec();
    let bottom = map.split_at(i).1.to_vec();

    if top.len() == 0 || bottom.len() == 0 {
        return false;
    }

    if top.len() > bottom.len() {
        let offset = top.len() - bottom.len();
        for (i, row) in top.iter().skip(offset).rev().enumerate() {
            if *row != bottom[i] {
                return false;
            }
        }
    } else {
        let offset = bottom.len() - top.len();
        for (i, row) in bottom.iter().rev().skip(offset).enumerate() {
            if *row != top[i] {
                return false;
            }
        }
    }

    true
}


fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut maps: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current_map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line == "" {
            maps.push(current_map);
            current_map = Vec::new();
        } else {
            current_map.push(line.chars().collect());
        }
    }
    maps.push(current_map);

    maps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_horizontal_mirror_at() {
        let map_1 = vec![
            vec!['.', '.', '#', '.'],
            vec!['.', '.', '#', '.'],
        ];

        let map_2 = vec![
            vec!['.', '.', '#', '.'],
            vec!['.', '#', '#', '.'],
            vec!['.', '#', '#', '.'],
        ];

        let map_3 = vec![
            vec!['.', '.', '#', '.'],
            vec!['.', '#', '#', '.'],
            vec!['.', '#', '#', '.'],
            vec!['.', '.', '#', '.'],
            vec!['.', '.', '.', '.'],
        ];
        let test_cases = vec![
            (&map_1, 0, false),
            (&map_1, 1, true),
            (&map_1, 2, false),
            (&map_2, 1, false),
            (&map_2, 2, true),
            (&map_2, 3, false),
            (&map_3, 1, false),
            (&map_3, 2, true),
        ];
        for (map, i, expected) in test_cases {
            assert_eq!(check_horizontal_mirror_at(map, i), expected, "failed on map: {:?}, i: {}", map, i);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
