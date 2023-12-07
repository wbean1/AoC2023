advent_of_code::solution!(3);

const SYMBOLS: &str = "@#$%&*-=+/";
const DIGITS: &str = "0123456789";

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let rows: Vec<Vec<&str>> = lines.map(|line| line.split("").collect()).collect();
    let mut numbers_selected: Vec<u32> = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let mut number_began = false;
        let mut number_is_selected = false;
        let mut current_number = 0;
        for (col_index, char) in row.iter().map(|c| *c).enumerate() {
            // if char == "" { continue }
            if "0123456789".contains(char) && char != "" {
                if number_began {
                    current_number = current_number * 10 + char.parse::<u32>().unwrap();
                } else {
                    number_began = true;
                    current_number = char.parse::<u32>().unwrap();
                }
                if !number_is_selected {
                    if (rows[row_index][col_index - 1] != "" && SYMBOLS.contains(rows[row_index][col_index - 1])) ||
                       (rows[row_index][col_index + 1] != "" && SYMBOLS.contains(rows[row_index][col_index + 1])) {
                        number_is_selected = true;
                    }
                    if row_index > 0 {
                        if (rows[row_index - 1][col_index] != "" && SYMBOLS.contains(rows[row_index - 1][col_index])) ||
                           (rows[row_index - 1][col_index - 1] != "" && SYMBOLS.contains(rows[row_index - 1][col_index - 1])) ||
                           (rows[row_index - 1][col_index + 1] != "" && SYMBOLS.contains(rows[row_index - 1][col_index + 1])) {
                            number_is_selected = true;
                        }
                    }
                    if row_index < rows.len() - 2 {
                        if (rows[row_index + 1][col_index] != "" && SYMBOLS.contains(rows[row_index + 1][col_index])) ||
                           (rows[row_index + 1][col_index - 1] != "" && SYMBOLS.contains(rows[row_index + 1][col_index - 1])) ||
                           (rows[row_index + 1][col_index + 1] != "" && SYMBOLS.contains(rows[row_index + 1][col_index + 1])) {
                            number_is_selected = true;
                        }
                    }
                }
            } else {
                if number_began {
                    // its now ended
                    if number_is_selected {
                        numbers_selected.push(current_number);
                    }
                    current_number = 0;
                    number_began = false;
                    number_is_selected = false;
                }
            }
        }
    }
    let current_sum = numbers_selected.iter().sum();
    Some(current_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let rows: Vec<Vec<&str>> = lines.map(|line| line.split("").collect()).collect();
    let gears = find_gears(&rows);
    let mut score = 0;
    for gear in gears {
        score += extract_score_for_gear(&rows, &gear)
    }
    Some(score)
}

fn extract_score_for_gear(m: &Vec<Vec<&str>>, coord: &(usize, usize)) -> u32 {
    let mut score = 1;
    // check left
    score *= extract_number_from_position(m, &(coord.0, coord.1 - 1));
    // check right
    score *= extract_number_from_position(m, &(coord.0, coord.1 + 1));
    // check up, if needed
    if coord.0 > 0 { score *= extract_number_from_position(m, &(coord.0 - 1 , coord.1)) };
    // check down, if needed
    if coord.0 < m.len() - 1 { score *= extract_number_from_position(m, &(coord.0 + 1 , coord.1)) };
    // check up diagonals, if needed
    if coord.0 > 0 && !DIGITS.contains(m[coord.0 - 1][coord.1]) {
        score *= extract_number_from_position(m, &(coord.0 - 1 , coord.1 - 1));
        score *= extract_number_from_position(m, &(coord.0 - 1 , coord.1 + 1));
    };
    // check down diagonals, if needed
    if coord.0 < m.len() - 1 && !DIGITS.contains(m[coord.0 + 1][coord.1]) {
        score *= extract_number_from_position(m, &(coord.0 + 1 , coord.1 - 1));
        score *= extract_number_from_position(m, &(coord.0 + 1 , coord.1 + 1));
    };
    score
}

fn extract_number_from_position(m: &Vec<Vec<&str>>, coord: &(usize, usize)) -> u32 {
    // if not a digit, return 1
    if m[coord.0][coord.1] != "" && !DIGITS.contains(m[coord.0][coord.1]) { return 1 };

    let mut number_string = m[coord.0][coord.1].to_owned();
    let mut current_row = coord.0;
    let mut current_col = coord.1;
    // while left is a digit, prepend
    while m[current_row][current_col - 1] != "" && DIGITS.contains(m[current_row][current_col - 1]) {
        number_string.insert_str(0, m[current_row][current_col - 1]);
        current_col -= 1;
    }

    current_row = coord.0;
    current_col = coord.1;
    // while right is a digit, append
    while m[current_row][current_col + 1] != "" && DIGITS.contains(m[current_row][current_col + 1]) {
        number_string.push_str(m[current_row][current_col + 1]);
        current_col += 1;
    }
    // convert string to u32 & return
    number_string.parse::<u32>().unwrap()
}

fn find_gears(m: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    for (row_index, row) in m.iter().enumerate() {
        for (col_index, item) in row.iter().map(|c| *c).enumerate() {
            if item != "*" { continue }
            // check non-diagonals first, to avoid double-counting them
            let mut adjacent_number_count = 0;
            // check left
            if DIGITS.contains(m[row_index][col_index - 1]) { adjacent_number_count += 1 };
            // check right
            if DIGITS.contains(m[row_index][col_index + 1]) { adjacent_number_count += 1 };
            // check up (if needed)
            if row_index > 0 && DIGITS.contains(m[row_index - 1][col_index]) { adjacent_number_count += 1 };
            // check down (if needed)
            if row_index < m.len() - 1 && DIGITS.contains(m[row_index + 1][col_index]) { adjacent_number_count += 1 };

            // if i have exactly two, oh god this sucks
            if adjacent_number_count == 2 { found.push((row_index, col_index)); continue };

            // if i have more than two, its not a gear
            if adjacent_number_count > 2 { continue };

            // i have less than two, and i need to check diagonals ONLY if up or down is not a digit...

            // check diagonals up?
            if row_index > 0 && !DIGITS.contains(m[row_index - 1][col_index]) {
                if DIGITS.contains(m[row_index - 1][col_index - 1]) { adjacent_number_count += 1 };
                if DIGITS.contains(m[row_index - 1][col_index + 1]) { adjacent_number_count += 1 };
            }

            // check diagonals down?
            if row_index < m.len() - 2 && !DIGITS.contains(m[row_index + 1][col_index]) {
                if DIGITS.contains(m[row_index + 1][col_index - 1]) { adjacent_number_count += 1 };
                if DIGITS.contains(m[row_index + 1][col_index + 1]) { adjacent_number_count += 1 };
            }

            // NOW if i have two, we good.
            if adjacent_number_count == 2 { found.push((row_index, col_index)); continue };
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_better() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4362));
    }

    #[test]
    fn test_part_one_better_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(4362));
    }

    #[test]
    fn test_part_one_better_3() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_find_gears() {
        let tests = HashMap::from([
            ("...123*456...", vec![(0,7)]),
            (
"...123...
...*.....
..4......
", vec![(1,4)]),
        ]);
        for (test, expected) in tests {
            let lines = test.split("\n");
            let rows: Vec<Vec<&str>> = lines.map(|line| line.split("").collect()).collect();
            let result = find_gears(&rows);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(467835));
    }
}
