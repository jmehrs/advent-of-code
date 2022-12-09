mod parse;

use std::collections::{HashMap, VecDeque};

pub use parse::{parse_input_part1, parse_input_part2, Instructions};

fn follow_instruction_part1(vec_map: &mut HashMap<u8, VecDeque<char>>, instruction: &Instructions) {
    for _ in 0..instruction.amount {
        if let Some(value) = vec_map.get_mut(&instruction.src).unwrap().pop_front() {
            vec_map.get_mut(&instruction.dst).unwrap().push_front(value);
        }
    }
}

fn follow_instruction_part2(vec_map: &mut HashMap<u8, VecDeque<char>>, instruction: &Instructions) {
    for i in 0..instruction.amount{
        if let Some(value) = vec_map.get_mut(&instruction.src).unwrap().pop_front() {
            vec_map.get_mut(&instruction.dst).unwrap().insert(i as usize, value);
        }
    }
}

fn follow_instructions<Spec>(
    vec_map: &mut HashMap<u8, VecDeque<char>>,
    instructions: &Vec<Instructions>,
    specification: Spec
)
where
    Spec: Fn(&mut HashMap<u8, VecDeque<char>>, &Instructions)
{
    for instruction in instructions {
        specification(vec_map, &instruction)
    }
}

fn get_vec_ends(vec_map: &HashMap<u8, VecDeque<char>>) -> String {
    (1..=vec_map.len())
        .map(|idx| vec_map.get(&(idx as u8)).unwrap()[0])
        .collect()
}

pub fn part1_solution1(input: &str) -> String {
    let (mut vec_map, instructions) = parse_input_part1(input);
    follow_instructions(&mut vec_map, &instructions, follow_instruction_part1);
    let result = get_vec_ends(&vec_map);
    result
}

pub fn part2_solution1(input: &str) -> String {
    let (mut vec_map, instructions) = parse_input_part1(input);
    follow_instructions(&mut vec_map, &instructions, follow_instruction_part2);
    let result = get_vec_ends(&vec_map);
    result
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
        assert_eq!(result, "MCD");
    }
}
