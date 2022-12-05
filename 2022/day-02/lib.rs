pub fn part1_solution1(input: &str) -> String {
    let max: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u32>().expect("Expected an integer"))
                .sum()
        })
        .max()
        .expect("No Max found");
    max.to_string()
}

pub fn part2_solution1(input: &str) -> String {
    let mut top3: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u32>().expect("Expected an integer"))
                .sum()
        }).collect();
    top3.sort();
    top3.reverse();
    let max: u32 = top3.iter().take(3).sum();
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn p1s1() {
        let result = part1_solution1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn p2s1() {
        let result = part2_solution1(INPUT);
        assert_eq!(result, "45000");
    }
}
