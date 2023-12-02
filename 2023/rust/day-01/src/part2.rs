pub fn process(input: &str) -> String {
    let output = input.lines().map(process_line).sum::<u32>();

    output.to_string()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        if let Some(first) = line.chars().nth(index) {
            if first.is_digit(10) {
                return first.to_digit(10);
            }
        }

        let line = &line[index..];
        for (word_index, &number) in NUMBERS.iter().enumerate() {
            if line.starts_with(number) {
                return Some((word_index + 1) as u32);
            }
        }

        None
    });

    let first = it.next().expect("number");
    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input));
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_process_line(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }
}
