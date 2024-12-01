#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut elements = line.split_whitespace();
            let first = elements.next().unwrap().parse::<i32>().unwrap();
            let second = elements.next().unwrap().parse::<i32>().unwrap();

            (first, second)
        })
        .unzip();

    first.sort();
    second.sort();
    let res: i32 = std::iter::zip(first, second)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
