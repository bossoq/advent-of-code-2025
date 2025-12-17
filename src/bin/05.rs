use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut final_fresh: HashSet<[usize; 2]> = HashSet::new();
    let mut fresh_count = 0;
    input.split("\n\n").enumerate().for_each(|(idx, group)| {
        if idx == 0 {
            let mut fresh: HashSet<[usize; 2]> = HashSet::new();
            group.lines().for_each(|line| {
                let (min, max) = line
                    .split_once('-')
                    .map(|(min, max)| {
                        (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap())
                    })
                    .unwrap();
                fresh.insert([min, max]);
            });
            let mut fresh_vec: Vec<[usize; 2]> = fresh.into_iter().collect();
            fresh_vec.sort();
            loop {
                let mut overlaps = false;
                let mut overlaps_count = 0;
                if let Some(first) = fresh_vec.iter().next().cloned() {
                    for elem in fresh_vec.clone().iter() {
                        if elem == &first {
                            continue;
                        }
                        if (elem[0] >= first[0] && elem[0] <= first[1])
                            || (elem[1] >= first[0] && elem[1] <= first[1])
                        {
                            overlaps = true;
                            overlaps_count += 1;
                        }
                        if overlaps {
                            let new_min = usize::min(first[0], elem[0]);
                            let new_max = usize::max(first[1], elem[1]);
                            fresh_vec.retain(|x| x != &first && x != elem);
                            fresh_vec.push([new_min, new_max]);
                            break;
                        }
                        overlaps = false;
                    }
                    if overlaps_count == 0 {
                        fresh_vec.retain(|x| x != &first);
                        final_fresh.insert(first);
                    }
                } else {
                    break;
                }
            }
        } else {
            group.lines().for_each(|line| {
                if let Ok(id) = line.parse::<usize>() {
                    for range in final_fresh.iter() {
                        if id >= range[0] && id <= range[1] {
                            fresh_count += 1;
                            break;
                        }
                    }
                }
            });
        }
    });
    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut final_fresh: HashSet<[u64; 2]> = HashSet::new();
    let mut fresh_count = 0;
    input.split("\n\n").enumerate().for_each(|(idx, group)| {
        if idx == 0 {
            let mut fresh: HashSet<[u64; 2]> = HashSet::new();
            group.lines().for_each(|line| {
                let (min, max) = line
                    .split_once('-')
                    .map(|(min, max)| (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap()))
                    .unwrap();
                fresh.insert([min, max]);
            });
            let mut fresh_vec: Vec<[u64; 2]> = fresh.into_iter().collect();
            fresh_vec.sort();
            loop {
                let mut overlaps = false;
                let mut overlaps_count = 0;
                if let Some(first) = fresh_vec.iter().next().cloned() {
                    for elem in fresh_vec.clone().iter() {
                        if elem == &first {
                            continue;
                        }
                        if (elem[0] >= first[0] && elem[0] <= first[1])
                            || (elem[1] >= first[0] && elem[1] <= first[1])
                        {
                            overlaps = true;
                            overlaps_count += 1;
                        }
                        if overlaps {
                            let new_min = u64::min(first[0], elem[0]);
                            let new_max = u64::max(first[1], elem[1]);
                            fresh_vec.retain(|x| x != &first && x != elem);
                            fresh_vec.push([new_min, new_max]);
                            break;
                        }
                        overlaps = false;
                    }
                    if overlaps_count == 0 {
                        fresh_vec.retain(|x| x != &first);
                        final_fresh.insert(first);
                    }
                } else {
                    break;
                }
            }
            for range in final_fresh.iter() {
                fresh_count += range[1] - range[0] + 1;
            }
        }
    });
    Some(fresh_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
