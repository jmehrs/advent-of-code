mod domain;
mod parse;

use domain::{RPS, RPSResult};
pub use parse::{parse_input_part1, parse_input_part2};



pub fn part1_solution1(input: &str) -> String {
    let input: Vec<(RPS, RPS)> = parse_input_part1(input);
    let result: u32 = input
        .iter()
        .map(|rps| rps.1.battle(rps.0).score())
        .sum();
    result.to_string()
}

pub fn part2_solution1(input: &str) -> String {
    let input: Vec<(RPS, &str)> = parse_input_part2(input);
    let result: u32 = input
        .iter()
        .map(|(rps, match_result)| {
            match *match_result {
                "X" => RPSResult::Lose({
                    match rps {
                        RPS::Rock => &RPS::Scissor,
                        RPS::Paper => &RPS::Rock,
                        RPS::Scissor => &RPS::Paper,
                    }
                }),
                "Y" => RPSResult::Draw({
                    match rps {
                        RPS::Rock => &RPS::Rock,
                        RPS::Paper => &RPS::Paper,
                        RPS::Scissor => &RPS::Scissor,
                    }
                }),
                "Z" => RPSResult::Win({
                    match rps {
                        RPS::Rock => &RPS::Paper,
                        RPS::Paper => &RPS::Scissor,
                        RPS::Scissor => &RPS::Rock,
                    }
                }),
                _ => panic!(),
            }.score()
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn p1s1() {
        let result = part1_solution1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn p2s1() {
        let result = part2_solution1(INPUT);
        assert_eq!(result, "12");
    }
}
