mod parse;

use std::ops::RangeInclusive;

pub use parse::{parse_input_part1, parse_input_part2};

fn left_fully_contains_right(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
   (range1.contains(range2.start()) && range1.contains(range2.end()))
   || (range1.contains(range2.start()) && range1.contains(range2.end()))
   
}

fn left_partially_contains_right(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
   range1.contains(range2.start()) || range1.contains(range2.end()) 
}

pub fn part1_solution1(input: &str) -> String {
    let input = parse_input_part1(input);
    let result = input
        .iter()
        .filter(|(r1, r2)| left_fully_contains_right(r1, r2) || left_fully_contains_right(r2, r1))
        .count();
    format!("{result:?}")
}

pub fn part2_solution1(input: &str) -> String {
    let input = parse_input_part1(input);
    let result = input
        .iter()
        .filter(|(r1, r2)| left_partially_contains_right(r1, r2) || left_partially_contains_right(r2, r1))
        .count();
    format!("{result:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn p1s1() {
        let result = part1_solution1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn p2s1() {
        let result = part2_solution1(INPUT);
        assert_eq!(result, "4");
    }
}
