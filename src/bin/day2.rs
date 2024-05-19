// im not too happy with my implementation, however this allowed me to learn some new patterns and
// practice traits and ordering
// maybe ill come back to this code someday and make it look better (no)
use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use aoc::{
    parse_file,
    Part::{self, *},
};

const PATH: &str = "./data/2.input";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Game {
    first_column: Move,
    second_column: Move,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        use Move::*;
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Ordering::Greater,
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        Ok(match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => unreachable!(),
        })
    }
}

impl Into<Outcome> for Move {
    fn into(self) -> Outcome {
        match self {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (first, second) = s.split_once(' ').unwrap();
        let first = Move::from_str(first)?;
        let second = Move::from_str(second)?;
        Ok(Game {
            first_column: first,
            second_column: second,
        })
    }
}

fn calculate_total_score(part: Part) -> Result<u32> {
    let games = parse_file::<Game>(&PATH)?;
    Ok(match part {
        Part1 => games
            .iter()
            .map(|game| {
                let score = match game.second_column.cmp(&game.first_column) {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                };
                game.second_column as u32 + score
            })
            .sum(),
        Part2 => games
            .iter()
            .map(|game| {
                let my_move = match (game.first_column, game.second_column.into()) {
                    (Move::Rock, Outcome::Win)
                    | (Move::Paper, Outcome::Draw)
                    | (Move::Scissors, Outcome::Lose) => Move::Paper,
                    (Move::Paper, Outcome::Win)
                    | (Move::Scissors, Outcome::Draw)
                    | (Move::Rock, Outcome::Lose) => Move::Scissors,
                    _ => Move::Rock,
                };

                my_move as u32 + Into::<Outcome>::into(game.second_column) as u32
            })
            .sum(),
    })
}

fn main() {
    println!("Part 1 - {:?}", calculate_total_score(Part1).unwrap());
    println!("Part 2 - {:?}", calculate_total_score(Part2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_1: u32 = 15337;
        let result_2: u32 = 11696;
        assert_eq!(result_1, calculate_total_score(Part1).unwrap());
        assert_eq!(result_2, calculate_total_score(Part2).unwrap());
    }
}
