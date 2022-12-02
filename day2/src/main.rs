use Outcome::*;
use RPS::*;

#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<char> for RPS {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Rock),
            'B' => Ok(Paper),
            'C' => Ok(Scissor),
            'X' => Ok(Rock),
            'Y' => Ok(Paper),
            'Z' => Ok(Scissor),
            _ => Err(()),
        }
    }
}

impl From<RPS> for u32 {
    fn from(rps: RPS) -> Self {
        match rps {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<Outcome> for u32 {
    fn from(o: Outcome) -> Self {
        match o {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Lose),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err(()),
        }
    }
}

/// Calculate the outcome from my move and their move
/// For part 1
fn outcome(them: RPS, me: RPS) -> Outcome {
    match (me, them) {
        (Rock, Scissor) | (Scissor, Paper) | (Paper, Rock) => Win,
        (Rock, Rock) | (Paper, Paper) | (Scissor, Scissor) => Draw,
        _ => Lose,
    }
}

/// Calculate my move from their move and the desired outcome
/// For part 2
fn my_move(their_move: RPS, o: Outcome) -> RPS {
    match (their_move, o) {
        (Rock, Win) => Paper,
        (Paper, Win) => Scissor,
        (Scissor, Win) => Rock,
        (Rock, Lose) => Scissor,
        (Paper, Lose) => Rock,
        (Scissor, Lose) => Paper,
        (x, Draw) => x,
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let part_1: u32 = input
        .lines()
        .map(|s| {
            let chars = s.chars().collect::<Vec<_>>();
            let their_move = RPS::try_from(chars[0]).unwrap();
            let my_move = RPS::try_from(chars[2]).unwrap();
            let outcome = outcome(their_move, my_move);
            Into::<u32>::into(my_move) + Into::<u32>::into(outcome)
        })
        .sum();

    println!("part 1: {}", part_1);

    let part_2: u32 = input
        .lines()
        .map(|s| {
            let chars = s.chars().collect::<Vec<_>>();
            let their_move = RPS::try_from(chars[0]).unwrap().into();
            let outcome = Outcome::try_from(chars[2]).unwrap();
            let my_move = my_move(their_move, outcome);
            Into::<u32>::into(my_move) + Into::<u32>::into(outcome)
        })
        .sum();

    println!("part 2: {}", part_2);
}
