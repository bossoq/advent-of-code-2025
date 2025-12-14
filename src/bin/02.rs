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
            let n_len = n_str.len();
            for i in 1..n_len {
                if n_len % i != 0 {
                    continue;
                }
                if n_str
                    .as_bytes()
                    .chunks(i)
                    .collect::<Vec<&[u8]>>()
                    .windows(2)
                    .all(|w| w[0] == w[1])
                {
                    result += n;
                    break;
                }
            }
        }
    });
    Some(result)
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
