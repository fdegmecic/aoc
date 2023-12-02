pub fn process(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|w| w.to_digit(10));
            let first = it.next().expect("number");

            match it.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
