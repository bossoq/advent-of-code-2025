advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos: i32 = 50;
    let mut password: u32 = 0;
    input.lines().for_each(|s| {
        let mut split_string = s.chars();
        if let Some(d) = split_string.next() {
            match d {
                'L' => {
                    split_string.as_str().parse::<i32>().ok().map(|n| pos -= n);
                }
                'R' => {
                    split_string.as_str().parse::<i32>().ok().map(|n| pos += n);
                }
                _ => { /* do nothing */ }
            }
            pos = pos % 100;
            if pos < 0 {
                pos += 100;
            }
            if pos == 0 {
                password += 1;
            }
        }
    });
    Some(password)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pos: i32 = 50;
    let mut password: u32 = 0;
    let mut zero: u32 = 1;
    input.lines().for_each(|s| {
        let mut split_string = s.chars();
        if let Some(d) = split_string.next() {
            match d {
                'L' => {
                    split_string.as_str().parse::<i32>().ok().map(|n| pos -= n);
                }
                'R' => {
                    split_string.as_str().parse::<i32>().ok().map(|n| pos += n);
                }
                _ => { /* do nothing */ }
            }
            if pos < 0 {
                password += (-pos / 100) as u32 + zero;
            } else if pos == 0 {
                password += zero;
            } else {
                password += (pos / 100) as u32;
            }
            zero = 1;
            pos = pos % 100;
            if pos < 0 {
                pos += 100;
            }
            if pos == 0 {
                zero = 0;
            }
        }
    });
    Some(password)
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
        assert_eq!(result, Some(6));
    }
}
