pub enum RPSResult<'a> {
    Lose(&'a RPS),
    Draw(&'a RPS),
    Win(&'a RPS),
}

impl RPSResult<'_> {
    pub fn score(&self) -> u32 {
        match self {
            RPSResult::Lose(rps) => (*rps.clone() as u32) + 0,
            RPSResult::Draw(rps) => (*rps.clone() as u32) + 3,
            RPSResult::Win(rps) => (*rps.clone() as u32) + 6,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RPS {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl From<&str> for RPS {
    fn from(item: &str) -> Self {
        match item {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissor,
            _ => panic!(),
        }
    }
}

impl RPS {
    pub fn battle(&self, other: RPS) -> RPSResult {
        match self {
            RPS::Rock => match other {
                RPS::Rock => RPSResult::Draw(&self),
                RPS::Paper => RPSResult::Lose(&self),
                RPS::Scissor => RPSResult::Win(&self),
            },
            RPS::Paper => match other {
                RPS::Rock => RPSResult::Win(&self),
                RPS::Paper => RPSResult::Draw(&self),
                RPS::Scissor => RPSResult::Lose(&self),
            },
            RPS::Scissor => match other {
                RPS::Rock => RPSResult::Lose(&self),
                RPS::Paper => RPSResult::Win(&self),
                RPS::Scissor => RPSResult::Draw(&self),
            },
        }
    }
}
