use cached::proc_macro::cached;
use cached::{Cached, SizedCache, Return};
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let tilted_map = tilt_map_north(&map);
    let load = compute_load(&tilted_map);

    Some(load)
}

fn tilt_map_north(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_map = map.clone();

    for (i, row) in map.iter().enumerate().skip(1) {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                if map[i-1][j] == '.' {
                    tilted_map[i-1][j] = 'O';
                    tilted_map[i][j] = '.';
                }
            }
        }
    }
    if tilted_map != *map {
        return tilt_map_north(&tilted_map);
    }

    tilted_map
}

fn tilt_map_south(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_map = map.clone();

    for (i, row) in map.iter().enumerate().rev().skip(1) {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                if map[i+1][j] == '.' {
                    tilted_map[i+1][j] = 'O';
                    tilted_map[i][j] = '.';
                }
            }
        }
    }
    if tilted_map != *map {
        return tilt_map_south(&tilted_map);
    }

    tilted_map
}

fn tilt_map_west(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_map = map.clone();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate().skip(1) {
            if *c == 'O' {
                if map[i][j-1] == '.' {
                    tilted_map[i][j-1] = 'O';
                    tilted_map[i][j] = '.';
                }
            }
        }
    }
    if tilted_map != *map {
        return tilt_map_west(&tilted_map);
    }

    tilted_map
}

fn tilt_map_east(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_map = map.clone();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate().rev().skip(1) {
            if *c == 'O' {
                if map[i][j+1] == '.' {
                    tilted_map[i][j+1] = 'O';
                    tilted_map[i][j] = '.';
                }
            }
        }
    }
    if tilted_map != *map {
        return tilt_map_east(&tilted_map);
    }

    tilted_map
}

fn compute_load(map: &Vec<Vec<char>>) -> u32 {
    let mut load = 0;
    for (i, row) in map.iter().enumerate() {
        for c in row.iter() {
            if *c == 'O' {
                load += (map.len()-i) as u32;
            }
        }
    }

    load
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut tilted_map = map.clone();
    let mut first_hit = 0;
    let mut second_hit = 0;
    for i in 1..1000000000 {
        let tilted_map_return = cycle(&tilted_map);
        tilted_map = tilted_map_return.value;
        if tilted_map_return.was_cached {
            if first_hit == 0 {
                first_hit = i;
                let mut cache1 = CYCLE.lock().unwrap();
                cache1.cache_clear();
            } else if second_hit == 0 {
                second_hit = i;
                let mut cache1 = CYCLE.lock().unwrap();
                cache1.cache_clear();
                break;
            }
        }
        // let cache1 = CYCLE.lock().unwrap();
        // println!("cycle: {}", i);
        // println!("  hits: {}, misses: {}", cache1.cache_hits().unwrap_or(0), cache1.cache_misses().unwrap_or(0));
    }
    let cycle_len = second_hit - first_hit - 1;
    let remainder_cycles = (1000000000 - first_hit-1) % cycle_len;
    for _ in 0..remainder_cycles {
        let tilted_map_return = cycle(&tilted_map);
        tilted_map = tilted_map_return.value;
    }

    Some(compute_load(&tilted_map))
}


#[cached(
    name = "CYCLE",
    type = "SizedCache<Vec<Vec<char>>, Return<Vec<Vec<char>>>>",
    create = "{ SizedCache::with_size(1000000) }",
    with_cached_flag = true,
)]
fn cycle(map: &Vec<Vec<char>>) -> Return<Vec<Vec<char>>> {
    let mut tilted_map = map.clone();

    tilted_map = tilt_map_north(&tilted_map);
    tilted_map = tilt_map_west(&tilted_map);
    tilted_map = tilt_map_south(&tilted_map);
    tilted_map = tilt_map_east(&tilted_map);

    Return::new(tilted_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
