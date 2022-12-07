mod parse;

pub use parse::{parse_input_part1, parse_input_part2};

fn char_intersections<'a>(str1: &'a str, str2: &'a str) -> impl Iterator<Item = char> + 'a {
    str1.chars()
        .filter_map(|str1_ch| str2.chars().find(|str2_ch| &str1_ch == str2_ch))
}

fn char_val(ch: char) -> u32 {
    let (idx, _) = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .find(|(_, c)| c == &ch)
        .unwrap();
    (idx as u32) + 1
}

pub fn part1_solution1(input: &str) -> String {
    let input = parse_input_part1(input);
    let result: u32 = input
        .iter()
        .map(|(str1, str2)| {
            char_intersections(str1, str2)
                .map(char_val)
                .next()
                .unwrap()
        })
        .sum();
    format!("{result:?}")
}

pub fn part2_solution1(input: &str) -> String {
    let input = parse_input_part2(input);
    let result: u32 = input
        .iter()
        .map(|group| {
            let str1_str2_matches = char_intersections(group[0], group[1])
                .collect::<String>();
            let result = char_intersections(&str1_str2_matches, group[2])
                .map(char_val)
                .next()
                .unwrap();
            result
        })
        .sum();
    format!("{result:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn p1s1() {
        let result = part1_solution1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn p2s1() {
        let result = part2_solution1(INPUT);
        assert_eq!(result, "70");
    }
}
