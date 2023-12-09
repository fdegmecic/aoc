use itertools::{Itertools, Position};
use nom::character::complete::newline;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

fn parse_reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

pub fn process(input: &str) -> i32 {
    let (_, reports) = parse_reports(input).expect("correct parsing");

    reports
        .iter()
        .map(|report| {
            let mut end_nums: Vec<i32> = vec![];
            let mut nums: Vec<i32> = report.to_vec();
            loop {
                if nums.iter().all(|num| num == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i32, &i32)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::Last | Position::Only => end_nums.push(*right),
                            _ => {}
                        }

                        right - left
                    })
                    .collect::<Vec<i32>>();
            }
            end_nums.iter().fold(0, |acc, num| acc + num)
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
