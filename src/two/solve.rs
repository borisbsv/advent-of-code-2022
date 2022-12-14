use std::collections::HashMap;

use crate::util::read::read;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

pub(crate) fn a(input: &str) -> i32 {
    let lines = read(input, |l| l.unwrap().chars().collect::<Vec<char>>());
    let mut score: i32 = 0;
    for l in lines {
        let lhs = l.first().unwrap();
        let rhs = l.last().unwrap();
        match lhs {
            'A' => match rhs {
                'X' => score += DRAW + ROCK, // rock
                'Y' => score += WIN + PAPER, // paper
                'Z' => score += SCISSORS,    // scissors
                _ => {}
            }, // rock
            'B' => match rhs {
                'X' => score += ROCK,           // rock
                'Y' => score += DRAW + PAPER,   // paper
                'Z' => score += WIN + SCISSORS, // scissors
                _ => {}
            }, // Paper
            'C' => match rhs {
                'X' => score += WIN + ROCK,      // rock
                'Y' => score += PAPER,           // paper
                'Z' => score += DRAW + SCISSORS, // scissors
                _ => {}
            }, // scissors
            _ => {}
        }
    }
    score
}

pub(crate) fn b(input: &str) -> i32 {
    let lines = read(input, |l| l.unwrap());

    let mapping = HashMap::from([
        ("A X", SCISSORS),
        ("A Y", DRAW + ROCK),
        ("A Z", WIN + PAPER),
        ("B X", ROCK),
        ("B Y", DRAW + PAPER),
        ("B Z", WIN + SCISSORS),
        ("C X", PAPER),
        ("C Y", DRAW + SCISSORS),
        ("C Z", WIN + ROCK),
    ]);
    let mut score: i32 = 0;
    for l in lines {
        score += mapping.get(&l.as_str()).unwrap();
    }
    score
}
