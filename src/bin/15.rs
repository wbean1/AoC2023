advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let segments = input.split(",").collect::<Vec<_>>();
    let mut total = 0;
    for seg in segments {
        total += hash(&seg.to_string());
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let segments = input.split(",").collect::<Vec<_>>();
    let mut boxes: Vec<Vec<(String, u32)>> = Vec::new();
    for _ in 0..256 {
        let empty_lense_vec: Vec<(String, u32)> = Vec::new();
        boxes.push(empty_lense_vec);
    }
    for seg in segments {
        let (label, operation, length): (String, bool, u32) = parse_segment(seg);
        let target_box = hash(&label);
        boxes = perform_operation_on_box(target_box, &label, operation, length, &boxes);
    }

    let total_focusing_power = compute_focusing_power(&boxes);

    Some(total_focusing_power)
}

fn hash(label: &String) -> u32 {
    let mut current_value = 0;
    for c in label.chars() {
        if c == '\n' { break; };
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn parse_segment(seg: &str) -> (String, bool, u32) {
    let label: String;
    let operation: bool;
    let length: u32;
    let try_equals = seg.split("=").nth(0).unwrap();
    if try_equals == seg { // no = in segment
        label = seg.split("-").nth(0).unwrap().to_string();
        operation = false;
        length = 0;
    } else {
        label = try_equals.to_string();
        operation = true;
        length = seg.strip_suffix("\n").unwrap_or(seg).split("=").nth(1).unwrap().parse().unwrap();
    }

    (label, operation, length)
}

fn perform_operation_on_box(target_box: u32, label: &String, operation: bool, length: u32, boxes: &Vec<Vec<(String, u32)>>) -> Vec<Vec<(String, u32)>> {
    let mut new_boxes = boxes.clone();
    let mut new_target_box = new_boxes[target_box as usize].clone();

    let mut label_found = false;
    let mut found_index: usize = 0;
    for (i, (existing_label, _)) in new_target_box.iter().enumerate() {
        if existing_label == label {
            label_found = true;
            found_index = i;
            break;
        }
    }
    // replace existing lense
    if operation {
        let new_lense = (label.clone(), length);
        if label_found {
            new_target_box[found_index] = new_lense;
        } else {
            new_target_box.push(new_lense)
        }
    } else {  // remove existing lense
        if label_found {
            new_target_box.remove(found_index);
        } // otherwise do nothing
    }
    // finally, replace the actual box with the modified one
    new_boxes[target_box as usize] = new_target_box;

    new_boxes
}

fn compute_focusing_power(boxes: &Vec<Vec<(String, u32)>>) -> u32 {
    let mut sum: u32 = 0;
    for (i, current_box) in boxes.iter().enumerate() {
        for (j, lense) in current_box.iter().enumerate() {
            sum += (i as u32 +1) * (j as u32 + 1) * lense.1;
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
