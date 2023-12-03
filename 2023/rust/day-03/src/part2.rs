use glam::IVec2;
use std::collections::HashSet;

struct EngineSchematic {
    part_numbers: Vec<PartNumber>,
    gears: HashSet<IVec2>,
}

struct PartNumber {
    value: u32,
    points: HashSet<IVec2>,
}

impl PartNumber {
    fn new(row: i32, col: i32, ch: char) -> Self {
        let points = HashSet::from([
            IVec2::new(row - 1, col - 1),
            IVec2::new(row, col - 1),
            IVec2::new(row + 1, col - 1), // left hand side
            IVec2::new(row - 1, col),
            IVec2::new(row + 1, col), // above and below
            IVec2::new(row - 1, col + 1),
            IVec2::new(row, col + 1),
            IVec2::new(row + 1, col + 1), // right hand side
        ]);

        Self {
            value: ch.to_digit(10).expect("a number"),
            points,
        }
    }

    fn add_digit(&mut self, row: i32, col: i32, ch: char) {
        self.value = self.value * 10 + ch.to_digit(10).expect("a number");
        self.points.extend([
            IVec2::new(row - 1, col + 1),
            IVec2::new(row, col + 1),
            IVec2::new(row + 1, col + 1),
        ]);
    }
}

impl EngineSchematic {
    fn new() -> Self {
        EngineSchematic {
            part_numbers: vec![],
            gears: HashSet::new(),
        }
    }

    fn parse_schematic(&mut self, input: &str) {
        let mut curr_number: Option<PartNumber> = None;

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, character)| {
                if character.is_ascii_digit() {
                    if let Some(ref mut num) = curr_number {
                        num.add_digit(row as i32, col as i32, character)
                    } else {
                        curr_number = Some(PartNumber::new(row as i32, col as i32, character))
                    }
                } else {
                    if let Some(num) = curr_number.take() {
                        self.part_numbers.push(num);
                    }
                    if character == '*' {
                        self.gears.insert(IVec2::new(row as i32, col as i32));
                    }
                }
            })
        });
    }
}

pub fn process(input: &str) -> u32 {
    let mut schematic = EngineSchematic::new();
    schematic.parse_schematic(input);

    schematic
        .part_numbers
        .iter()
        .filter(|num| {
            schematic
                .gears
                .iter()
                .filter(|gear| num.points.contains(gear))
                .count()
                == 2
        })
        .map(|num| num.value)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input))
    }
}
