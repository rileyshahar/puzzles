/// Solve the problem.

#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Sizzors,
}

use Play::{Paper, Rock, Sizzors};

impl Play {
    const fn strategy(self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (Rock, Win) | (Paper, Draw) | (Sizzors, Loss) => Paper,
            (Rock, Draw) | (Paper, Loss) | (Sizzors, Win) => Rock,
            (Rock, Loss) | (Paper, Win) | (Sizzors, Draw) => Sizzors,
        }
    }

    #[allow(dead_code)]
    const fn against(self, other: Self) -> Outcome {
        match (self, other) {
            (Rock, Sizzors) | (Paper, Rock) | (Sizzors, Paper) => Win,
            (Rock, Rock) | (Paper, Paper) | (Sizzors, Sizzors) => Draw,
            (Rock, Paper) | (Paper, Sizzors) | (Sizzors, Rock) => Loss,
        }
    }

    const fn score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Sizzors => 3,
        }
    }
}

impl TryFrom<char> for Play {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Sizzors,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    const fn score(self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

use Outcome::{Draw, Loss, Win};

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => return Err(()),
        })
    }
}

struct Round {
    opp: Play,
    desire: Outcome,
}

impl Round {
    const fn us(self) -> Play {
        self.opp.strategy(self.desire)
    }

    const fn score(self) -> u32 {
        self.desire.score() + self.us().score()
    }
}

impl std::str::FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();

        let opp = c.next().ok_or(())?.try_into()?;
        c.next();
        let desire = c.next().ok_or(())?.try_into()?;

        Ok(Self { opp, desire })
    }
}

fn solve_for(input: &'static str) -> u32 {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse::<Round>)
        .map(|r| r.expect("inputs are valid rounds"))
        .map(Round::score)
        .sum()
}

super::example!(12, "22-02");
super::problem!(u32, "22-02");
