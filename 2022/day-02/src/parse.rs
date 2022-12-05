use crate::RPS;

pub fn parse_input_part1(input: &str) -> Vec<(RPS, RPS)> {
    input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .map(|items| (items.0.into(), items.1.into()))
                .unwrap()
        })
        .collect()
}


pub fn parse_input_part2(input: &str) -> Vec<(RPS, &str)> {
    input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .map(|items| (items.0.into(), items.1))
                .unwrap()
        })
        .collect()
}