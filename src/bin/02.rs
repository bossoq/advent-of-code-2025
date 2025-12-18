use std::collections::{HashMap, HashSet};
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    input.trim().split(',').for_each(|s| {
        let num = s
            .split('-')
            .map(|n| match n.parse::<u64>() {
                Ok(val) => val,
                Err(e) => panic!("Failed to parse number {}: {}", n, e),
            })
            .collect::<Vec<u64>>();
        let start = num.iter().next().unwrap();
        let end = num.iter().last().unwrap();
        for n in *start..=*end {
            let n_str = n.to_string();
            if n_str.len() % 2 == 0 {
                let (left, right) = n_str.split_at(n_str.len() / 2);
                if left == right {
                    result += n;
                }
            }
        }
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut factors_collection: HashMap<u32, HashSet<u32>> = HashMap::new();
    // let mut factors_collection: HashMap<u32, HashSet<u32>> = HashMap::from([
    //     (2, HashSet::from([1])),
    //     (3, HashSet::from([1])),
    //     (4, HashSet::from([1, 2])),
    //     (5, HashSet::from([1])),
    //     (6, HashSet::from([1, 2, 3])),
    //     (7, HashSet::from([1])),
    //     (8, HashSet::from([1, 2, 4])),
    //     (9, HashSet::from([1, 3])),
    //     (10, HashSet::from([1, 2, 5])),
    // ]);
    let mut cache_table: HashMap<[u32; 2], (u64, u64, u64)> = HashMap::new();
    // let mut cache_table: HashMap<[u32; 2], (u64, u64, u64)> = HashMap::from([
    //     ([2, 1], (1, 9, 11)),
    //     ([3, 1], (1, 9, 111)),
    //     ([4, 1], (1, 9, 1111)),
    //     ([4, 2], (10, 99, 101)),
    //     ([5, 1], (1, 9, 11111)),
    //     ([6, 1], (1, 9, 111111)),
    //     ([6, 2], (10, 99, 10101)),
    //     ([6, 3], (100, 999, 1001)),
    //     ([7, 1], (1, 9, 1111111)),
    //     ([8, 1], (1, 9, 11111111)),
    //     ([8, 2], (10, 99, 1010101)),
    //     ([8, 4], (1000, 9999, 10001)),
    //     ([9, 1], (1, 9, 111111111)),
    //     ([9, 3], (100, 999, 1001001)),
    //     ([10, 1], (1, 9, 1111111111)),
    //     ([10, 2], (10, 99, 101010101)),
    //     ([10, 5], (10000, 99999, 100001)),
    // ]);
    let mut result: u64 = 0;
    input.trim().split(',').for_each(|s| {
        let num = s
            .split('-')
            .map(|n| match n.parse::<u64>() {
                Ok(val) => val,
                Err(e) => panic!("Failed to parse number {}: {}", n, e),
            })
            .collect::<Vec<u64>>();
        let start = num.iter().next().unwrap();
        let end = num.iter().last().unwrap();
        for n in *start..=*end {
            let n_len = n.to_string().len() as u32;
            if n_len == 1 {
                continue;
            }
            let factors = match factors_collection.get(&n_len) {
                Some(facs) => facs,
                None => {
                    let mut facs: HashSet<u32> = HashSet::new();
                    for i in 1..n_len {
                        if n_len % i == 0 {
                            facs.insert(i);
                        }
                    }
                    factors_collection.insert(n_len, facs);
                    factors_collection.get(&n_len).unwrap()
                }
            };
            for f in factors {
                let (start, end, multiplier) = match cache_table.get(&[n_len, *f]) {
                    Some(vals) => *vals,
                    None => {
                        if f == &1 {
                            let start = 1u64;
                            let end = 9u64;
                            let multiplier = "1".repeat(n_len as usize).parse::<u64>().unwrap();
                            cache_table.insert([n_len, *f], (start, end, multiplier));
                            (start, end, multiplier)
                        } else {
                            let start = 10u64.pow(f - 1);
                            let end = 10u64.pow(*f) - 1;
                            let n_group = n_len / f;
                            let multiplier = start
                                .to_string()
                                .repeat(n_group as usize)
                                .parse::<u64>()
                                .unwrap()
                                / start;
                            cache_table.insert([n_len, *f], (start, end, multiplier));
                            (start, end, multiplier)
                        }
                    }
                };
                if n % multiplier == 0 && n / multiplier >= start && n / multiplier <= end {
                    result += n;
                    break;
                }
            }
        }
    });
    Some(result)
    // let mut result: u64 = 0;
    // input.trim().split(',').for_each(|s| {
    //     let num = s
    //         .split('-')
    //         .map(|n| match n.parse::<u64>() {
    //             Ok(val) => val,
    //             Err(e) => panic!("Failed to parse number {}: {}", n, e),
    //         })
    //         .collect::<Vec<u64>>();
    //     let start = num.iter().next().unwrap();
    //     let end = num.iter().last().unwrap();
    //     for n in *start..=*end {
    //         let n_str = n.to_string();
    //         let n_len = n_str.len();
    //         for i in 1..n_len {
    //             if n_len % i != 0 {
    //                 continue;
    //             }
    //             if n_str
    //                 .as_bytes()
    //                 .chunks(i)
    //                 .collect::<Vec<&[u8]>>()
    //                 .windows(2)
    //                 .all(|w| w[0] == w[1])
    //             {
    //                 result += n;
    //                 break;
    //             }
    //         }
    //     }
    // });
    // Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
