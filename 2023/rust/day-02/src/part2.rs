use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use nom::character::complete::digit1;

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    fn minimum_colors_power(&self) -> u32 {
        let map: HashMap<&str, u32> = HashMap::new();
        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| {
                            *v = (*v).max(cube.amount);
                        })
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
    }
}

fn parse_cubes(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), parse_cubes)(input)?;
    Ok((input, cubes))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), parse_rounds))(input)?;
    Ok((input, Game { rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, parse_game)(input)?;
    Ok((input, games))
}

pub fn process(input: &str) -> u32 {
    let (_, games) = parse_games(input).expect("should parse games");
    games.iter().map(Game::minimum_colors_power).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input));
    }
}
