advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let plan: Vec<(&str, usize)> = input.lines().map(
        |line| (line.split_whitespace().nth(0).unwrap(), line.split_whitespace().nth(1).unwrap().parse().unwrap())
    ).collect();

    // the challenge with this one is you have no idea how large the map needs to be?
    // i think we can just sum up the number of 'R' and 'D', then multiply that by two
    // since we don't know if we'll go generally Up or Down, Left or Right.
    // and then also start smack dab in the middle?
    let mut east_west_length: usize = 1;
    let mut north_south_length: usize = 1;
    for i in plan.iter() {
        if i.0 == "R" { east_west_length += i.1; }
        if i.0 == "D" { north_south_length += i.1; }
    }

    east_west_length *= 2;
    north_south_length *= 2;

    let mut map: Vec<Vec<bool>> = Vec::new();
    for _ in 0..north_south_length {
        map.push(vec![false; east_west_length]);
    }

    // start in the middle, and iterate thru plan...
    let mut current_point = (east_west_length/2, north_south_length/2);
    map[current_point.1][current_point.0] = true;
    for i in plan.iter() {
        if i.0 == "R" {
            for _ in 0..i.1 {
                current_point = (current_point.0 + 1, current_point.1);
                map[current_point.1][current_point.0] = true;
            }
        }
        if i.0 == "L" {
            for _ in 0..i.1 {
                current_point = (current_point.0 - 1, current_point.1);
                map[current_point.1][current_point.0] = true;
            }
        }
        if i.0 == "U" {
            for _ in 0..i.1 {
                current_point = (current_point.0, current_point.1 - 1);
                map[current_point.1][current_point.0] = true;
            }
        }
        if i.0 == "D" {
            for _ in 0..i.1 {
                current_point = (current_point.0, current_point.1 + 1);
                map[current_point.1][current_point.0] = true;
            }
        }
    }
    println!("map before fill:");
    print_map(&map);

    // now fill in the map for contained points
    map = fill(&map);

    println!("map after fill:");
    print_map(&map);

    // and sum up the points.
    Some(score(&map))
}

fn print_map(map: &Vec<Vec<bool>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn score(map: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                score += 1;
            }
        }
    }

    score
}

fn fill(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_map = map.clone();
    for y in 1..map.len()-1 {
        for x in 1..map[0].len()-1 {
            if should_fill(&map, x, y) {
                new_map[y][x] = true;
            }
        }
    }

    new_map
}

fn should_fill(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    if map[y][x] == true { return true; }
    if x == 0 || y == 0 || x == map[0].len()-1 || y == map.len()-1 { return false; }

    bounded_up(&map, x, y) &&
    bounded_down(&map, x, y) &&
    bounded_left(&map, x, y) &&
    bounded_right(&map, x, y) &&
    should_fill(&map, x+1, y) &&
    should_fill(&map, x-1, y) &&
    should_fill(&map, x, y+1) &&
    should_fill(&map, x, y-1)
}

fn bounded_up(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    for i in 0..y {
        if map[i][x] {
            return true;
        }
    }

    false
}

fn bounded_down(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    for i in y+1..map.len() {
        if map[i][x] {
            return true;
        }
    }

    false
}

fn bounded_left(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    for i in 0..x {
        if map[y][i] {
            return true;
        }
    }

    false
}

fn bounded_right(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    for i in x+1..map[0].len() {
        if map[y][i] {
            return true;
        }
    }

    false
}


pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_up() {
        let map = vec![
            vec![false, false, true, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, false, false, true],
            vec![false, false, true, false],
        ];
        assert_eq!(bounded_up(&map, 2, 3), true);
        assert_eq!(bounded_up(&map, 0, 0), false);
        assert_eq!(bounded_up(&map, 1, 4), false);
    }

    #[test]
    fn test_bounded_down() {
        let map = vec![
            vec![false, false, true, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, false, false, true],
            vec![false, false, true, false],
        ];
        assert_eq!(bounded_down(&map, 2, 3), true);
        assert_eq!(bounded_down(&map, 0, 0), true);
        assert_eq!(bounded_down(&map, 3, 4), false);
        assert_eq!(bounded_down(&map, 1, 0), false);
    }

    #[test]
    fn test_bounded_left() {
        let map = vec![
            vec![false, false, true, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, false, false, true],
            vec![false, false, true, false],
        ];
        assert_eq!(bounded_left(&map, 2, 3), true);
        assert_eq!(bounded_left(&map, 0, 0), false);
        assert_eq!(bounded_left(&map, 3, 4), true);
        assert_eq!(bounded_left(&map, 3, 2), false);
    }

    #[test]
    fn test_bounded_right() {
        let map = vec![
            vec![false, false, true, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, false, false, true],
            vec![false, false, true, false],
        ];
        assert_eq!(bounded_right(&map, 2, 3), true);
        assert_eq!(bounded_right(&map, 0, 0), true);
        assert_eq!(bounded_right(&map, 3, 4), false);
        assert_eq!(bounded_right(&map, 0, 2), false);
    }

    #[test]
    fn test_fill() {
        let unfilled_map = vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![false, true, true, false],
        ];
        let filled_map = vec![
            vec![false, true, true, false],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![false, true, true, false],
        ];
        assert_eq!(filled_map, fill(&unfilled_map));
    }

    #[test]
    fn test_score(){
        let map = vec![
            vec![false, false, true, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, false, false, true],
            vec![false, false, true, false],
        ];
        assert_eq!(4, score(&map));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_one_again() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(169));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
