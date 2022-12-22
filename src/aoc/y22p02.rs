/// Solve the problem.

#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Sizzors,
}

use Play::{Paper, Rock, Sizzors};

impl Play {
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
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Sizzors,
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

struct Round {
    us: Play,
    opp: Play,
}

impl Round {
    const fn outcome(self) -> Outcome {
        self.us.against(self.opp)
    }

    const fn score(self) -> u32 {
        self.us.score() + self.outcome().score()
    }
}

impl std::str::FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();

        let opp = c.next().ok_or(())?.try_into()?;
        c.next();
        let us = c.next().ok_or(())?.try_into()?;

        Ok(Self { us, opp })
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

super::example!(15, "22-02");
super::problem!(u32, "22-02");
