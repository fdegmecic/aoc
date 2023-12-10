use day_09::part1_1::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{}", result);
}
