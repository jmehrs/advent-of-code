fn extract_code(line: &str) -> u32 {
    let nums: Vec<u32> = line.chars().filter(|ch| {
        ch.is_numeric()
    })
    .filter_map(|num_char| {
        num_char.to_digit(10)
    })
    .collect();

    let num_amt = nums.len();
    if num_amt == 0 {
        0
    } else if num_amt == 1 {
        nums.first().unwrap() * 10 + nums.first().unwrap()
    } else {
        nums.first().unwrap() * 10 + nums.last().unwrap()
    }
}

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(extract_code)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

    #[test]
    fn p1s1() {
        let result = process(INPUT);
        assert_eq!(result, 142);
    }
}
