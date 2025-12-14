use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    let map = parse_input(input);
    for pos in map.iter() {
        let adjacents = find_adjacents(*pos);
        let mut count = 0;
        for adj in adjacents {
            if map.contains(&adj) {
                count += 1;
            }
            if count == 4 {
                continue;
            }
        }
        if count < 4 {
            ans += 1;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    let mut map = parse_input(input);
    let mut remove = HashSet::new();
    loop {
        for pos in map.iter() {
            let adjacents = find_adjacents(*pos);
            let mut count = 0;
            for adj in adjacents {
                if map.contains(&adj) {
                    count += 1;
                }
                if count == 4 {
                    continue;
                }
            }
            if count < 4 {
                remove.insert(*pos);
                ans += 1;
            }
        }
        if remove.is_empty() {
            break;
        }
        for pos in remove.drain() {
            map.remove(&pos);
        }
    }
    Some(ans)
}

fn parse_input(input: &str) -> HashSet<[usize; 2]> {
    let mut map = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            map.insert([i, j]);
        }
    }
    map
}

fn find_adjacents(pos: [usize; 2]) -> Vec<[usize; 2]> {
    let mut adjacents = Vec::new();
    let directions = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    for dir in directions.iter() {
        let new_pos = [pos[0] as isize + dir[0], pos[1] as isize + dir[1]];
        if new_pos[0] >= 0 && new_pos[1] >= 0 {
            adjacents.push([new_pos[0] as usize, new_pos[1] as usize]);
        }
    }
    adjacents
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
        assert_eq!(result, Some(43));
    }
}
