use std::collections::HashSet;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    input.lines().for_each(|line| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(idx, text)| {
                if let Some(num) = text.parse::<u64>().ok() {
                    if problems.len() <= idx {
                        problems.push(Vec::new());
                    }
                    problems[idx].push(num);
                } else {
                    if operations.len() <= idx {
                        operations.push(' ');
                    }
                    operations[idx] = text.chars().next().unwrap_or(' ');
                }
            });
    });
    let result: u64 = problems
        .iter()
        .enumerate()
        .map(|(idx, nums)| {
            let op = operations[idx];
            match op {
                '+' => nums.iter().sum(),
                '*' => nums.iter().product(),
                _ => 0,
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut problems: Vec<Vec<char>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    input.lines().enumerate().for_each(|(idx, line)| {
        if idx == input.lines().count() - 1 {
            line.split_whitespace().enumerate().for_each(|(idx, text)| {
                if operations.len() <= idx {
                    operations.push(' ');
                }
                operations[idx] = text.chars().next().unwrap_or(' ');
            });
            return;
        }
        problems.push(line.chars().collect());
    });
    let mut common_space: HashSet<usize> = HashSet::new();
    problems.iter().enumerate().for_each(|(ridx, line)| {
        if ridx == 0 {
            line.iter().enumerate().for_each(|(cidx, ch)| {
                if ch.is_whitespace() {
                    common_space.insert(cidx);
                }
            });
        } else {
            common_space = common_space
                .intersection(
                    &line
                        .iter()
                        .enumerate()
                        .filter_map(|(cidx, ch)| if ch.is_whitespace() { Some(cidx) } else { None })
                        .collect(),
                )
                .copied()
                .collect();
        }
    });
    let mut common_space: Vec<usize> = common_space.into_iter().collect();
    common_space.sort();
    let mut problems_adj: Vec<Vec<Vec<char>>> = Vec::new();
    problems.iter().for_each(|line| {
        let mut current_word = Vec::new();
        let mut real_cidx = 0;
        for (cidx, ch) in line.iter().enumerate() {
            if common_space.contains(&cidx) {
                if !current_word.is_empty() {
                    if problems_adj.len() <= real_cidx {
                        problems_adj.push(Vec::new());
                    }
                    problems_adj[real_cidx].push(current_word.clone());
                    real_cidx += 1;
                    current_word.clear();
                }
            } else {
                current_word.push(*ch);
            }
        }
        if !current_word.is_empty() {
            if problems_adj.len() <= real_cidx {
                problems_adj.push(Vec::new());
            }
            problems_adj[real_cidx].push(current_word.clone());
        }
    });
    let result: u64 = problems_adj
        .iter()
        .enumerate()
        .map(|(idx, prob)| {
            let max_len = prob.iter().map(|word| word.len()).max().unwrap_or(0);
            let op = operations[idx];
            let mut num_arr = Vec::new();
            for i in 0..max_len {
                let mut tmp_string = String::new();
                prob.iter().for_each(|ch_vec| {
                    if let Some(ch) = ch_vec.get(i) {
                        if ch.is_numeric() {
                            tmp_string.push(*ch);
                        }
                    }
                });
                if !tmp_string.is_empty() {
                    if let Ok(num) = tmp_string.parse::<u64>() {
                        num_arr.push(num);
                    }
                }
            }
            match op {
                '+' => num_arr.iter().sum(),
                '*' => num_arr.iter().product(),
                _ => 0,
            }
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
