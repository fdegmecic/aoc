use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn number(input: &str) -> IResult<&str, u64> {
    is_not("123456789")
        .precedes(
            separated_list1(space1, digit1)
                .map(|list| list.join("").parse::<u64>().expect("valid number")),
        )
        .parse(input)
}

fn parse_times(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(number, line_ending, number).parse(input)
}

pub fn process(input: &str) -> usize {
    let (_, (time, record_distance)) = parse_times(input).expect("valid parsing");

    (1..time)
        .filter_map(|hold| {
            let result_distance = hold * (time - hold);

            if result_distance > record_distance {
                return Some(hold);
            } else {
                return None;
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      71530
Distance:  940200";
        assert_eq!(288, process(input));
    }
}
