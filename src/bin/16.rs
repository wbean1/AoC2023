use std::collections::HashMap;
advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    position: (usize, usize),
    direction: char,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Some(run_simulation(&map, (0,0), 'E'))
}

fn process_step(map: &Vec<Vec<char>>, beams: &Vec<Beam>) -> Vec<Beam> {
    let mut new_beams: Vec<Beam> = Vec::new();

    // for each beam
    for beam in beams.iter() {
        // switch current position to energized
        let current_position = beam.position;

        // inspect position, change direction & split if needed, push to new beams
        let position_char: char = map[current_position.1][current_position.0];

        if position_char == '.' {
            new_beams.push(Beam { position: current_position, direction: beam.direction });
        } else if position_char == '/' {
            let new_beam_direction = match beam.direction {
                    'N' => 'E',
                    'S' => 'W',
                    'E' => 'N',
                    _  => 'S',
            };
            new_beams.push(Beam { position: current_position, direction: new_beam_direction });
        } else if position_char == '\\' {
            let new_beam_direction = match beam.direction {
                    'N' => 'W',
                    'S' => 'E',
                    'E' => 'S',
                    _  => 'N',
            };
            new_beams.push(Beam { position: current_position, direction: new_beam_direction });
        } else if position_char == '-' {
            if beam.direction == 'S' || beam.direction == 'N' {
                    new_beams.push(Beam { position: current_position, direction: 'E' });
                    new_beams.push(Beam { position: current_position, direction: 'W' });
            } else {
                    new_beams.push(Beam { position: current_position, direction: beam.direction });
            }
        } else if position_char == '|' {
            if beam.direction == 'E' || beam.direction == 'W' {
                new_beams.push(Beam { position: current_position, direction: 'N' });
                new_beams.push(Beam { position: current_position, direction: 'S' });
            } else {
                new_beams.push(Beam { position: current_position, direction: beam.direction });
            }
        }
    }
    let mut moved_beams: Vec<Beam> = Vec::new();
    for beam in new_beams.iter() {
        // move direction - NEED TO CHECK BOUNDARIES OF MAP
        let current_position = beam.position;
        let mut new_position = current_position;
        if beam.direction == 'E' {
            if current_position.0 < map[0].len()-1 {
                new_position = (current_position.0 + 1, current_position.1);
            }
        } else if beam.direction == 'W' {
            if current_position.0 > 0 {
                new_position = (current_position.0 - 1, current_position.1);
            }
        } else if beam.direction == 'N' {
            if current_position.1 > 0 {
                new_position = (current_position.0, current_position.1 - 1);
            }
        } else if beam.direction == 'S' {
            if current_position.1 < map.len()-1 {
                new_position = (current_position.0, current_position.1 + 1);
            }
        }
        if new_position != current_position {
            moved_beams.push(Beam { position: new_position, direction: beam.direction });
        }
    }

    moved_beams
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut max_energy: u32 = 0;

    // try top row heading S
    for x in 0..map[0].len()-1 {
        let energy = run_simulation(&map, (x, 0), 'S');
        if energy > max_energy {
            max_energy = energy;
        }
    }
    // try bottom row heading N
    for x in 0..map[0].len()-1 {
        let energy = run_simulation(&map, (x, map.len()-1), 'N');
        if energy > max_energy {
            max_energy = energy;
        }
    }
    // try left col heading E
    for y in 0..map.len()-1 {
        let energy = run_simulation(&map, (0, y), 'E');
        if energy > max_energy {
            max_energy = energy;
        }
    }
    // try right col heading W
    for y in 0..map.len()-1 {
        let energy = run_simulation(&map, (map[0].len()-1, y), 'W');
        if energy > max_energy {
            max_energy = energy;
        }
    }

    Some(max_energy)
}

fn run_simulation(map: &Vec<Vec<char>>, initial_position: (usize, usize), initial_direction: char) -> u32 {

    let mut beams: Vec<Beam> = vec![
        Beam { position: initial_position, direction: initial_direction },
    ];

    let mut energies: HashMap<(usize,usize), bool> = HashMap::new();
    let mut seen_beams: HashMap<Beam, bool> = HashMap::new();
    let mut unseen_beams: Vec<Beam> = Vec::new();

    let mut none_found_iterations = 0;

    // HACK: this is an arbitrary number of iterations :shrug:
    while none_found_iterations < map[0].len() + map.len() {
        for beam in beams.iter() {
            if seen_beams.insert(beam.clone(), true) == None {
                unseen_beams.push(beam.clone());
            }
            if energies.insert(beam.position, true) == None {
                none_found_iterations = 0;
            }
        }
        none_found_iterations += 1;
        beams = process_step(&map, &unseen_beams);
    }

    energies.keys().len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
