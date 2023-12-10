use itertools::Itertools;
use nom::character::complete::newline;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};
use std::ops::Not;

fn parse_reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

pub fn process(input: &str) -> i32 {
    let (_, reports) = parse_reports(input).expect("correct parsing");

    reports
        .into_iter()
        .map(|report| {
            std::iter::successors(Some(report), |nums| {
                nums.iter().all(|num| num == &0).not().then_some(
                    nums.iter()
                        .tuple_windows::<(&i32, &i32)>()
                        .map(|(left, right)| right - left)
                        .collect(),
                )
            })
            .map(|v| *v.last().unwrap())
            .sum::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input));
    }
}
