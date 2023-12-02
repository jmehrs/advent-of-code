use day_02::part_1::process;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Could not read input.txt");
    println!("{}", process(&input, 12, 13, 14));
}
