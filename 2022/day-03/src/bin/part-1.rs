use day_03::part1_solution1;
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Could not read input.txt");
    println!("{}", part1_solution1(&input));
}
