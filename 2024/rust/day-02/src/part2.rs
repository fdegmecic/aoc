use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut res = 0;
    input.lines().for_each(|line| {
        let mut safe = false;
        let numbers = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for i in 0..numbers.len() {
            let mut copy = numbers.clone();
            copy.remove(i);
            dbg!(copy.clone());
            safe = check(copy);
            if safe {
                break;
            }
        }

        if safe {
            res += 1;
        }
    });

    Ok(res.to_string())
}

fn check(list: Vec<i32>) -> bool {
    let windows = list
        .into_iter()
        .tuple_windows::<(i32, i32)>()
        .map(|(l, r)| l - r)
        .collect::<Vec<_>>();

    windows.iter().all(|diff| (1..3).contains(diff))
        || windows.iter().all(|diff| (-3..1).contains(diff))
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
