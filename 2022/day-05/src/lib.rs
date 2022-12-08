mod parse;

use std::{collections::{HashMap, VecDeque}, slice::SliceIndex};

pub use parse::{parse_input_part1, parse_input_part2, Instructions};

fn follow_instruction(vec_map: &mut HashMap<u8, VecDeque<char>>, instruction: &Instructions) {
    if let Some(value) = vec_map.get_mut(&instruction.src).unwrap().pop_front() {
        vec_map.get_mut(&instruction.dst).unwrap().push_front(value);
    }
}

fn follow_instructions(vec_map: &mut HashMap<u8, VecDeque<char>>, instructions: &Vec<Instructions>) {
    for instruction in instructions {
        for _ in 0..instruction.amount {
            follow_instruction(vec_map, instruction)
        }
    }
}

fn get_vec_ends(vec_map: &HashMap<u8, VecDeque<char>>) -> String {
    (1..=vec_map.len())
        .map(|idx| {
            vec_map.get(&(idx as u8)).unwrap()[0]
        })
        .collect()
}

pub fn part1_solution1(input: &str) -> String {
    let (mut vec_map, instructions) = parse_input_part1(input);
    follow_instructions(&mut vec_map, &instructions);
    let result = get_vec_ends(&vec_map);
    result
}

pub fn part2_solution1(input: &str) -> String {
    let input = parse_input_part2(input);
    let result = input;
    format!("{result:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn p1s1() {
        let result = part1_solution1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn p2s1() {
        let result = part2_solution1(INPUT);
        assert_eq!(result, "70");
    }
}
