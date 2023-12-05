advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut current_sum = 0;
    for line in lines {
        let mut current_ticket_score = 0;
        let (winning_nums, have_nums) = parse_line(line);
        for num in winning_nums {
            if have_nums.contains(&num) {
                if current_ticket_score == 0 {
                    current_ticket_score = 1;
                } else {
                    current_ticket_score *= 2;
                }
            }
        }
        current_sum += current_ticket_score;
    };
    Some(current_sum)
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut winning_nums: Vec<u32> = Vec::new();
    let mut have_nums: Vec<u32> = Vec::new();
    if line == "" { return (winning_nums, have_nums) };
    let temp_string = line.split(": ").nth(1).unwrap();
    let winning_string = temp_string.split(" | ").nth(0).unwrap();
    let have_string = temp_string.split(" | ").nth(1).unwrap();

    winning_nums = winning_string.split(" ").map(|s| s.trim())
    .filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect();
    have_nums = have_string.split(" ").map(|s| s.trim())
    .filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect();

    (winning_nums, have_nums)
}
// cards_you_have:
// 1 -> 1  *wins 4
// 2 -> 1+1 *wins 2
// 3 -> 1+1 +2 *wins 3
// 4 -> 1+1 +2 +4
// 5 -> 1+1 +4
// 6 -> 1 +4
pub fn part_two(input: &str) -> Option<u32> {
    let mut number_of_each_card: Vec<u32> = Vec::new();
    let lines = input.split("\n");
    let len = lines.clone().count();
    for _ in 0..len-1 {
        number_of_each_card.push(1);
    };
    for (ticket_number, line) in lines.enumerate() {
        let (winning_nums, have_nums) = parse_line(line);
        let number_of_wins = count_wins(&winning_nums, &have_nums);
        let mut i = 1;
        while i <= number_of_wins {
            number_of_each_card[ticket_number + usize::try_from(i).unwrap()] += number_of_each_card[ticket_number];
            i += 1;
        };
    };
    Some(number_of_each_card.iter().sum::<u32>())
}

fn count_wins(winning_nums: &Vec<u32>, have_nums: &Vec<u32>) -> u32 {
    let mut number_of_wins = 0;
    for num in winning_nums {
        if have_nums.contains(&num) {
            number_of_wins += 1;
        }
    };
    number_of_wins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
