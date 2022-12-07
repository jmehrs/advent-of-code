use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character;
use nom::character::complete::newline;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::multi::separated_list1;

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = character::complete::u32(input)?;
    Ok((input, start..=end))
}

fn range_pair(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, pair) = separated_pair(range, tag(","), range)(input)?;
    Ok((input, pair))
}

fn range_pairs(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (_, pairs) = separated_list1(newline, range_pair)(input).unwrap();
    pairs
}

pub fn parse_input_part1(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    range_pairs(&input)
}

pub fn parse_input_part2(input: &str) -> &str {
    input
}
