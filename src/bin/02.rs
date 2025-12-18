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
    let mut cache_table: HashMap<[u32; 2], (u64, u64, u64)> = HashMap::new();
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
