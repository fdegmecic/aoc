use std::{collections::HashSet, vec};

pub fn process(input: &str) -> usize {
    let mut result = vec![];
    for line in input.lines() {
        let (_, rest) = line.split_once(':').unwrap();

        let (winning, numbers) = rest
            .split_once('|')
            .map(|(winning, numbers)| {
                let winning_set: HashSet<u32> = winning
                    .split_whitespace()
                    .filter_map(|num_str| num_str.parse::<u32>().ok())
                    .collect();

                let nums_set: HashSet<u32> = numbers
                    .split_whitespace()
                    .filter_map(|num_str| num_str.parse::<u32>().ok())
                    .collect();

                (winning_set, nums_set)
            })
            .unwrap();

        result.push((winning, numbers))
    }

    let mut res = vec![1usize; result.len()];

    for (idx, (win, num)) in result.iter().enumerate() {
        let count = win.intersection(&num).count();

        for i in idx + 1..idx + 1 + count {
            res[i] += res[idx];
        }
    }

    res.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input));
    }
}
