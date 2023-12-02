use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

struct Game {
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = delimited(
        tag("Game "),
        digit1,
        tag(": "),
    )(input)?;

    let mut game = Game {
        max_red: 0,
        max_green: 0,
        max_blue: 0,
    };
    for round in input.split("; ") {
        for item in round.split(", ") {
            let (_, (amt, color)) = separated_pair(
                map_res(digit1, |s: &str| s.parse::<usize>()),
                tag(" "),
                alphanumeric1,
            )(item)?;
            if color == "red" {
                if amt > game.max_red {
                    game.max_red = amt;
                }
            } else if color == "green" {
                if amt > game.max_green {
                    game.max_green = amt;
                }
            } else {
                if amt > game.max_blue {
                    game.max_blue = amt;
                }
            }
        }
    }

    Ok((input, game))
}

fn get_game_power(line: &str) -> Option<usize> {
    if let Ok((
        _,
        Game {
            max_red,
            max_green,
            max_blue,
            ..
        },
    )) = parse_game(line)
    {
        return Some(max_red * max_green * max_blue);
    }
    None
}

pub fn process(input: &str) -> usize {
    input
        .lines()
        .filter_map(get_game_power)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn p1s1() {
        let result = process(INPUT);
        assert_eq!(result, 2286);
    }
}
