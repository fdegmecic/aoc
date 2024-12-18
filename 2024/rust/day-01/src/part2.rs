use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (first, second): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| {
            let mut elements = line.split_whitespace();
            let first = elements.next().unwrap().parse::<usize>().unwrap();
            let second = elements.next().unwrap().parse::<usize>().unwrap();

            (first, second)
        })
        .unzip();

    let frequencies = second.iter().counts();
    let res = first
        .iter()
        .fold(0, |acc, num| acc + num * frequencies.get(num).unwrap_or(&0));

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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
