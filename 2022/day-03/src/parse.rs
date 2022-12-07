use std::iter;

pub fn parse_input_part1(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect()
}

// pub fn parse_input_part2(input: &str) -> Vec<&[(&str, &str)]> {
pub fn parse_input_part2(input: &str) -> Vec<Vec<&str>> {
    let lines = input
        .lines()
        .collect::<Vec<&str>>();
    let line_total = lines.len(); 
    let idxs = (0..line_total).step_by(3);
    let mut group_indexs = idxs.clone().chain(iter::once(line_total));
    _ = group_indexs.next();
    idxs
        .zip(group_indexs)
        .map(|(start, end)| {
            lines[start..=(end-1)].to_owned()
        })
        .collect()
}
