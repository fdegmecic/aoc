use day_01::part2_1::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{}", result);
}
