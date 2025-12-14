advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut joltages: Vec<u32> = Vec::new();
    input.lines().for_each(|b| {
        let jolts = b
            .chars()
            .map(|j| j.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let size = jolts.len();
        let mut sub_j = [0u8; 2];
        for (idx, j) in jolts.iter().enumerate() {
            if j > &sub_j[0] && idx < size - 1 {
                sub_j[0] = *j;
                sub_j[1] = 0;
            } else if j > &sub_j[1] && idx < size - 0 {
                sub_j[1] = *j;
            }
        }
        joltages.push(sub_j[0] as u32 * 10 + sub_j[1] as u32 * 1);
    });
    Some(joltages.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltages: Vec<u64> = Vec::new();
    input.lines().for_each(|b| {
        let jolts = b
            .chars()
            .map(|j| j.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let size = jolts.len();
        let mut sub_j = [0u8; 12];
        for (idx, j) in jolts.iter().enumerate() {
            if j > &sub_j[0] && idx < size - 11 {
                sub_j[0] = *j;
                for k in 1..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[1] && idx < size - 10 {
                sub_j[1] = *j;
                for k in 2..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[2] && idx < size - 9 {
                sub_j[2] = *j;
                for k in 3..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[3] && idx < size - 8 {
                sub_j[3] = *j;
                for k in 4..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[4] && idx < size - 7 {
                sub_j[4] = *j;
                for k in 5..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[5] && idx < size - 6 {
                sub_j[5] = *j;
                for k in 6..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[6] && idx < size - 5 {
                sub_j[6] = *j;
                for k in 7..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[7] && idx < size - 4 {
                sub_j[7] = *j;
                for k in 8..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[8] && idx < size - 3 {
                sub_j[8] = *j;
                for k in 9..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[9] && idx < size - 2 {
                sub_j[9] = *j;
                for k in 10..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[10] && idx < size - 1 {
                sub_j[10] = *j;
                for k in 11..12 {
                    sub_j[k] = 0;
                }
            } else if j > &sub_j[11] && idx < size - 0 {
                sub_j[11] = *j;
            }
        }
        joltages.push(
            sub_j[0] as u64 * 100_000_000_000
                + sub_j[1] as u64 * 10_000_000_000
                + sub_j[2] as u64 * 1_000_000_000
                + sub_j[3] as u64 * 100_000_000
                + sub_j[4] as u64 * 10_000_000
                + sub_j[5] as u64 * 1_000_000
                + sub_j[6] as u64 * 100_000
                + sub_j[7] as u64 * 10_000
                + sub_j[8] as u64 * 1_000
                + sub_j[9] as u64 * 100
                + sub_j[10] as u64 * 10
                + sub_j[11] as u64 * 1,
        );
    });
    Some(joltages.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
