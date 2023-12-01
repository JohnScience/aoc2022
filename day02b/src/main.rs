use std::{convert::Infallible, str::FromStr};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

struct EnemyTurn(Choice);

struct PlayerTurn(Choice);

struct Round {
    enemy_turn: EnemyTurn,
    player_turn: PlayerTurn,
}

impl FromStr for Round {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let enemy_turn = match s[0] - b'A' {
            0 => EnemyTurn(Choice::Rock),
            1 => EnemyTurn(Choice::Paper),
            2 => EnemyTurn(Choice::Scissors),
            _ => unreachable!(),
        };
        // s[1] is whitespace
        let outcome = match s[2] - b'X' {
            0 => Outcome::Loss,
            1 => Outcome::Draw,
            2 => Outcome::Win,
            _ => unreachable!(),
        };
        let player_turn = PlayerTurn::deduce(&enemy_turn, &outcome);
        Ok(Round {
            enemy_turn,
            player_turn,
        })
    }
}

impl Round {
    fn win_score_component(&self) -> u8 {
        match (self.player_turn.0, self.enemy_turn.0) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => 6,
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => 0,
            _draw => 3,
        }
    }

    fn score(&self) -> u8 {
        let player_score_component = self.player_turn.score();
        let win_score_component = self.win_score_component();
        player_score_component + win_score_component
    }
}

impl PlayerTurn {
    fn score(&self) -> u8 {
        println!("{:?}", self.0);
        self.0 as u8 + 1
    }

    fn deduce(enemy_turn: &EnemyTurn, player_outcome: &Outcome) -> Self {
        let choice = match (enemy_turn.0, player_outcome) {
            (Choice::Paper, Outcome::Loss)
            | (Choice::Rock, Outcome::Draw)
            | (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Scissors, Outcome::Loss)
            | (Choice::Paper, Outcome::Draw)
            | (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Rock, Outcome::Loss)
            | (Choice::Scissors, Outcome::Draw)
            | (Choice::Paper, Outcome::Win) => Choice::Scissors,
        };
        PlayerTurn(choice)
    }
}

// macro_rules! print_round {
//     ($round:literal) => {
//         let r: Round = $round.parse().unwrap();
//         println!("{}", r.score());
//     };
// }

fn main() {
    let input = std::fs::read_to_string(r"C:\Users\USER\Documents\github\aoc2022\day02a\input.txt")
        .unwrap();
    let sum = input
        .lines()
        .map(|l| l.parse::<Round>().unwrap())
        .map(|round| round.score())
        .map(|score| score as u32)
        .sum::<u32>();
    println!("{}", sum);
}
