use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let hands_to_bids:HashMap<&str, u32> = parse_input(input);
    let hands = hands_to_bids.keys().cloned().collect::<Vec<&str>>();
    let ordered_hands: Vec<&str> = order_hands(hands, false);
    let mut winnings = 0;
    for (index, hand) in ordered_hands.iter().enumerate() {
        let bid = hands_to_bids.get(hand).unwrap();
        winnings += bid * (u32::try_from(index).unwrap() + 1);
    }

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands_to_bids:HashMap<&str, u32> = parse_input(input);
    let hands = hands_to_bids.keys().cloned().collect::<Vec<&str>>();
    let ordered_hands: Vec<&str> = order_hands(hands, true);
    let mut winnings = 0;
    for (index, hand) in ordered_hands.iter().enumerate() {
        let bid = hands_to_bids.get(hand).unwrap();
        winnings += bid * (u32::try_from(index).unwrap() + 1);
    }

    Some(winnings)
}

fn order_hands(hands: Vec<&str>, jokers: bool) -> Vec<&str> {
    let mut hands_to_scores = HashMap::new();
    for hand in hands.iter().copied() {
        let score = score_hand(hand, jokers);
        hands_to_scores.insert(hand, score);
    }
    let mut hash_vec = Vec::new();
    for (hand, score) in hands_to_scores.iter() {
        hash_vec.push((*hand, score));
    }
    hash_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ordered_hands = Vec::new();
    for (hand, _) in hash_vec {
        ordered_hands.push(hand);
    }

    ordered_hands
}

fn score_hand(hand: &str, jokers: bool) -> u64 {
    let num_jokers: u32 = match jokers {
        true => count_jokers(hand),
        _ => 0,
    };
    let score: u64;
    if is_five_of_kind(hand, num_jokers){
        score = 60000000000 + score_by_card_values(hand, jokers);
    } else if is_four_of_kind(hand, num_jokers){
        score = 50000000000 + score_by_card_values(hand, jokers);
    } else if is_full_house(hand, num_jokers){
        score = 40000000000 + score_by_card_values(hand, jokers);
    } else if is_three_of_kind(hand, num_jokers){
        score = 30000000000 + score_by_card_values(hand, jokers);
    } else if is_two_pair(hand, num_jokers){
        score = 20000000000 + score_by_card_values(hand, jokers);
    } else if is_one_pair(hand, num_jokers){
        score = 10000000000 + score_by_card_values(hand, jokers);
    } else {
        score = score_by_card_values(hand, jokers);
    }

    score
}

fn count_jokers(hand: &str) -> u32 {
    let mut count = 0;
    for char in hand.chars() {
        if char == 'J' {
            count += 1;
        }
    }

    count
}

fn is_five_of_kind(hand: &str, num_jokers: u32) -> bool {
    if num_jokers == 1 {
        return is_four_of_kind(hand, 0);
    } else if num_jokers == 2 {
        return is_three_of_kind(hand, 0);
    } else if num_jokers == 3 {
        return is_two_pair(hand, 0);
    } else if num_jokers == 4 {
        return true;
    }
    let mut chars = hand.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();
    let fourth = chars.next().unwrap();
    let fifth = chars.next().unwrap();
    first == second && second == third && third == fourth && fourth == fifth
}

fn is_four_of_kind(hand: &str, num_jokers: u32) -> bool {
    if num_jokers == 1 {
        return is_three_of_kind(hand, 0);
    } else if num_jokers == 2 {
        return is_two_pair(hand, 0);
    } else if num_jokers >= 3 {
        return true;
    }
    let sorted_hand = hand.chars().sorted().collect::<String>();
    let mut chars = sorted_hand.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();
    let fourth = chars.next().unwrap();
    let fifth = chars.next().unwrap();
    (first == second && second == third && third == fourth) ||
        (second == third && third == fourth && fourth == fifth)
}

fn is_full_house(hand: &str, num_jokers: u32) -> bool {
    if num_jokers == 1 {
        return is_two_pair(hand, 0) || is_three_of_kind(hand, 0);
    } else if num_jokers == 2 {
        return is_two_pair(hand, 0);
    } else if num_jokers >= 3 {
        return true;
    }
    let sorted_hand = hand.chars().sorted().collect::<String>();
    let mut chars = sorted_hand.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();
    let fourth = chars.next().unwrap();
    let fifth = chars.next().unwrap();
    (first == second && second == third && fourth == fifth) ||
        (first == second && third == fourth && fourth == fifth)
}

fn is_three_of_kind(hand: &str, num_jokers: u32) -> bool {
    if num_jokers == 1 {
        return is_one_pair(hand, 0);
    } else if num_jokers >= 2 {
        return true;
    }
    let sorted_hand = hand.chars().sorted().collect::<String>();
    let mut chars = sorted_hand.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();
    let fourth = chars.next().unwrap();
    let fifth = chars.next().unwrap();
    (first == second && second == third) ||
        (second == third && third == fourth) ||
        (third == fourth && fourth == fifth)
}

fn is_two_pair(hand: &str, num_jokers: u32) -> bool {
    if num_jokers == 1 {
        return is_one_pair(hand, 0);
    } else if num_jokers >= 2 {
        return true;
    }
    let sorted_hand = hand.chars().sorted().collect::<String>();
    let mut chars = sorted_hand.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();
    let fourth = chars.next().unwrap();
    let fifth = chars.next().unwrap();
    (first == second && third == fourth) ||
        (first == second && fourth == fifth) ||
        (second == third && fourth == fifth)
}

fn is_one_pair(hand: &str, num_jokers: u32) -> bool {
    if num_jokers >= 1 {
        return true;
    }
    let sorted_hand = hand.chars().sorted().collect::<String>();
    let mut chars = sorted_hand.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();
    let fourth = chars.next().unwrap();
    let fifth = chars.next().unwrap();
    first == second || second == third || third == fourth || fourth == fifth
}

// 1 * 5th card value
// 100 * 4th card value
// 10000 * 3rd card value
// 1000000 * 2nd card value
// 100000000 * 1st card value
fn score_by_card_values(hand: &str, jokers: bool) -> u64 {
    let card_values: HashMap<char, u64> = match jokers {
        false => HashMap::from([
                    ('A', 14),
                    ('K', 13),
                    ('Q', 12),
                    ('J', 11),
                    ('T', 10),
                    ('9', 9),
                    ('8', 8),
                    ('7', 7),
                    ('6', 6),
                    ('5', 5),
                    ('4', 4),
                    ('3', 3),
                    ('2', 2),
                ]),
        _ => HashMap::from([
                    ('A', 14),
                    ('K', 13),
                    ('Q', 12),
                    ('J', 1),
                    ('T', 10),
                    ('9', 9),
                    ('8', 8),
                    ('7', 7),
                    ('6', 6),
                    ('5', 5),
                    ('4', 4),
                    ('3', 3),
                    ('2', 2),
                ]),
    };
    let mut score = 0;
    score += card_values.get(&hand.chars().nth(0).unwrap()).unwrap() * 100000000;
    score += card_values.get(&hand.chars().nth(1).unwrap()).unwrap() * 1000000;
    score += card_values.get(&hand.chars().nth(2).unwrap()).unwrap() * 10000;
    score += card_values.get(&hand.chars().nth(3).unwrap()).unwrap() * 100;
    score += card_values.get(&hand.chars().nth(4).unwrap()).unwrap();
    score
}

fn parse_input(input: &str) -> HashMap<&str, u32> {
    let mut hands_to_bids = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(" ");
        let hand = split.next().unwrap();
        let bid = split.next().unwrap().parse::<u32>().unwrap();
        hands_to_bids.insert(hand, bid);
    }
    hands_to_bids
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_five_of_kind() {
        let test_cases: HashMap<(&str, u32), bool> = HashMap::from([
            (("AAAAA", 0), true),
            (("AJAAA", 0), false),
            (("AJAAA", 1), true),
            (("AJAAQ", 1), false),
            (("AJAJA", 2), true),
            (("AJJJA", 3), true),
            (("AQJJJ", 3), false),
            (("AJJJJ", 4), true),
            (("JJJJJ", 5), true),
        ]);
        for ((hand, num_jokers), expected) in test_cases {
            let result = is_five_of_kind(hand, num_jokers);
            assert_eq!(result, expected);
        }
    }
    // etc...

    #[test]
    fn test_order_hands_basic() {
        let test_cases: HashMap<(Vec<&str>, bool), Vec<&str>> = HashMap::from([
            ((vec!["A2345", "23456", "34567"], false), vec!["23456", "34567", "A2345"]),
            ((vec!["A2345", "J3456", "34567"], false), vec!["34567", "J3456", "A2345"]),
            ((vec!["A2345", "J3456", "34567"], true), vec!["34567", "A2345", "J3456"]),
        ]);
        for ((hands, use_jokers), expected) in test_cases {
            let result = order_hands(hands, use_jokers);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_input() {
        let test_case = "32T3K 765\n\
                               T55J5 684\n";
        let result = parse_input(test_case);
        let expected = HashMap::from([
            ("32T3K", 765),
            ("T55J5", 684),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
