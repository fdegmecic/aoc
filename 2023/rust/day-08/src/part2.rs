use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parser(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

pub fn process(input: &str) -> u64 {
    let (_, (directions, map)) = parser(input).expect("to parse");
    let mut directions_it = directions.iter().cycle();

    let start_positions = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    println!("{start_positions:?}");

    let mut paths = vec![];
    for &position in start_positions {
        let mut current = position;
        let mut total = 0;

        while !current.ends_with('Z') {
            let (left, right) = map.get(current).expect("");
            total += 1;

            match directions_it.next().expect("") {
                Direction::Left => current = left,
                Direction::Right => current = right,
            }
        }
        paths.push(total);
    }

    paths.iter().fold(1, |acc, &x| lcm(acc, x))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    b * (a / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(2, process(input));
    }
}
