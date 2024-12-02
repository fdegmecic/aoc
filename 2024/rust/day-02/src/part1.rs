use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let solution = input
        .lines()
        .filter(|line| {
            let numbers = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            check(numbers)
        })
        .count();

    Ok(solution.to_string())
}

fn check(list: Vec<i32>) -> bool {
    let mut increase = false;
    let mut decrease = false;
    let test = list
        .into_iter()
        .tuple_windows::<(i32, i32)>()
        .collect::<Vec<_>>();
    let res = test
        .iter()
        .filter(|window| {
            let first = window.0;
            let second = window.1;
            let diff = first - second;

            if !(1..=3).contains(&diff.abs()) {
                return false;
            }

            if first > second {
                increase = true
            } else {
                decrease = true
            }

            increase != decrease
        })
        .count();

    res == test.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
