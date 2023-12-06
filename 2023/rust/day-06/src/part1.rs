use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_times(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(numbers, line_ending, numbers).parse(input)
}

pub fn process(input: &str) -> usize {
    let (_, (times, distances)) = parse_times(input).expect("valid parsing");

    times
        .into_iter()
        .zip(distances)
        .map(|(time, record_distance)| {
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
        })
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input));
    }
}
