use day_02::part2_solution1;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Could not read input.txt");
    println!("{}", part2_solution1(&input));
}
