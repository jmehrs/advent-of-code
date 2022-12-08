use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace1, newline, satisfy, space0, space1};
use nom::character::{self, is_alphabetic};
use nom::combinator::opt;
use nom::error::ParseError;
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;


#[derive(Debug, Clone)]
pub struct Instructions {
    pub amount: u8,
    pub src: u8,
    pub dst: u8,
}

impl Instructions {
    fn new(amount: u8, src: u8, dst: u8) -> Self {
        Self { amount, src, dst }
    }
}

fn single_space(input: &str) -> IResult<&str, char> {
    satisfy(|chr| chr == ' ')(input)
}

fn opt_bracketed_char(input: &str) -> IResult<&str, char> {
    let (input, chr) = delimited(
        alt((tag("["), tag(" "))),
        alt((satisfy(|chr| is_alphabetic(chr as u8)), single_space)),
        alt((tag("]"), tag(" "))),
    )(input)?;
    Ok((input, chr))
}

fn opt_chars(input: &str) -> IResult<&str, Vec<char>> {
    let (input, chrs) = separated_list1(tag(" "), opt_bracketed_char)(input)?;
    Ok((input, chrs))
}

fn char_vecs(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, pairs) = separated_list1(newline, opt_chars)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, pairs))
}

fn vec_ids(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, vec_id_str) = take_until("\n\n")(input)?;
    let vec_ids = vec_id_str
        .trim()
        .split_whitespace()
        .map(|chr| chr.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let (input, _) = tag("\n\n")(input)?;
    Ok((input, vec_ids))
}

fn single_instruction(input: &str) -> IResult<&str, Instructions> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = character::complete::u8(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, src) = character::complete::u8(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, dst) = character::complete::u8(input)?;

    Ok((input, Instructions::new(amount, src, dst)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instructions>> {
    let (input, instructions) = separated_list1(newline, single_instruction)(input)?;
    Ok((input, instructions))
}

fn transpose(ch_vec: Vec<Vec<char>>) -> Vec<VecDeque<char>> {
    let len = ch_vec[0].len();
    let mut iter: Vec<_> = ch_vec.into_iter().map(|inner_vec| inner_vec.into_iter()).collect();
    (0..len)
        .map(|_| {
            iter
                .iter_mut()
                .map(|n| n.next().unwrap())
                .filter(|chr| chr != &' ')
                .collect::<VecDeque<char>>()
        })
        .collect()
}

fn initial_state(input: &str) -> IResult<&str, HashMap<u8, VecDeque<char>>> {
    let (input, ch_vec) = char_vecs(input).unwrap();
    let (input, vec_ids) = vec_ids(input).unwrap();
    let ch_vec = transpose(ch_vec);

    let mut vec_map = HashMap::new();
    let _: Vec<_> = vec_ids
        .iter()
        .zip(ch_vec.into_iter())
        .map(|(id, ch_vec)| {
            vec_map.insert(*id, ch_vec)
        })
        .collect();
    Ok((input, vec_map))
}

pub fn parse_input_part1(input: &str) -> (HashMap<u8, VecDeque<char>>, Vec<Instructions>) {
    let (input, vec_map) = initial_state(input).unwrap();
    let (_, instructions) = instructions(input).unwrap();
    (vec_map, instructions)
}

pub fn parse_input_part2(input: &str) -> &str {
    input
}
