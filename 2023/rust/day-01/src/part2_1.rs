use aho_corasick::AhoCorasick;

pub fn process(input: &str) -> u32 {
    let ac = AhoCorasick::new(NUMBERS).unwrap();

    let mut total = 0;
    for line in input.lines() {
        let mut it = ac.find_overlapping_iter(line);
        let first = it.next().unwrap().pattern().as_usize() / 2 + 1;
        total += match it.last() {
            Some(possible_match) => first * 10 + possible_match.pattern().as_usize() / 2 + 1,
            None => first * 10 + first,
        }
    }

    total as u32
}

const NUMBERS: [&str; 18] = [
    "one", "1", "two", "2", "three", "3", "four", "4",
    "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input));
    }
}
