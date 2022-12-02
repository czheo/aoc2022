use std::io;
use std::cmp::{Ordering, Ord, PartialOrd};

#[derive(PartialEq, Eq)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Action::Rock, Action::Paper) => Ordering::Less,
            (Action::Paper, Action::Scissors) => Ordering::Less,
            (Action::Scissors, Action::Rock) => Ordering::Less,
            _ if self == other => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

fn main() {
    let mut score = 0;
    for l in io::stdin().lines() {
        let line = l.unwrap();
        let mut it = line.split_ascii_whitespace().take(2);
        let x = parse(it.next().unwrap());
        let y = parse(it.next().unwrap());
        if x < y {
            score += 6;
        } else if x == y {
            score += 3;
        }
        score += y as u32;
    }
    println!("{}", score);
}

fn parse(s : &str) -> Action {
    match s {
        "A" | "X" => Action::Rock,
        "B" | "Y" => Action::Paper,
        "C" | "Z" => Action::Scissors,
        _ => panic!("Unknown input: {}", s)
    }
}
