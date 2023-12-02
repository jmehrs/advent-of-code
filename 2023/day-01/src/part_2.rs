fn extract_code(line: &str) -> u32 {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut nums: Vec<u32> = vec![];

    for i in 0..len {
        // Check if current character(byte) is a number
        if &bytes[i] == &('1' as u8) {nums.push(1); continue}
        else if &bytes[i] == &('2' as u8) {nums.push(2); continue}
        else if &bytes[i] == &('3' as u8) {nums.push(3); continue}
        else if &bytes[i] == &('4' as u8) {nums.push(4); continue}
        else if &bytes[i] == &('5' as u8) {nums.push(5); continue}
        else if &bytes[i] == &('6' as u8) {nums.push(6); continue}
        else if &bytes[i] == &('7' as u8) {nums.push(7); continue}
        else if &bytes[i] == &('8' as u8) {nums.push(8); continue}
        else if &bytes[i] == &('9' as u8) {nums.push(9); continue}

        // Look ahead for characters that spell out a number
        let bytes_remaining = len - i;
        if bytes_remaining >= 5 {
            if &bytes[i..i+5] == &['t' as u8, 'h' as u8, 'r' as u8, 'e' as u8, 'e' as u8] {nums.push(3); continue}
            else if &bytes[i..i+5] == &['s' as u8, 'e' as u8, 'v' as u8, 'e' as u8, 'n' as u8] {nums.push(7); continue}
            else if &bytes[i..i+5] == &['e' as u8, 'i' as u8, 'g' as u8, 'h' as u8, 't' as u8] {nums.push(8); continue}
        } 
        if bytes_remaining >= 4 {
            if &bytes[i..i+4] == &['f' as u8, 'o' as u8, 'u' as u8, 'r' as u8]  {nums.push(4); continue}
            else if &bytes[i..i+4] == &['f' as u8, 'i' as u8, 'v' as u8, 'e' as u8]  {nums.push(5); continue}
            else if &bytes[i..i+4] == &['n' as u8, 'i' as u8, 'n' as u8, 'e' as u8]  {nums.push(9); continue}
        }
        if bytes_remaining >= 3 {
            if &bytes[i..i+3] == &['o' as u8, 'n' as u8, 'e' as u8] {nums.push(1); continue}
            else if &bytes[i..i+3] == &['t' as u8, 'w' as u8, 'o' as u8] {nums.push(2); continue}
            else if &bytes[i..i+3] == &['s' as u8, 'i' as u8, 'x' as u8] {nums.push(6); continue}
        }
    }
    
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

    const INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn p2s1() {
        let result = process(INPUT);
        assert_eq!(result, 281);
    }
}
