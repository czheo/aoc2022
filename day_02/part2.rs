use std::io;
use std::cmp::{Ordering};


#[derive(Copy, Clone)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

const ACTION_ARRAY: [Action; 3] = [Action::Rock, Action::Paper, Action::Scissors];

fn main() {
    let mut score = 0;
    for l in io::stdin().lines() {
        let line = l.unwrap();
        let mut it = line.split_ascii_whitespace().take(2);
        let x = parse_action(it.next().unwrap());
        let y = parse_result(it.next().unwrap());
        if y == Ordering::Equal{
            score += 3;
        } else if y == Ordering::Greater {
            score += 6;
        }
        score += get_action(x, y) as u32;
    }
    println!("{}", score);
}

fn get_action(action: Action, result: Ordering) -> Action {
    match result {
        Ordering::Equal => action,
        Ordering::Less => ACTION_ARRAY[(action as i32 - 2).rem_euclid(3) as usize],
        Ordering::Greater => ACTION_ARRAY[(action as i32).rem_euclid(3) as usize],
    }
}

fn parse_action(s: &str) -> Action {
    match s {
        "A" => Action::Rock,
        "B" => Action::Paper,
        "C" => Action::Scissors,
        _ => panic!("Unknown input: {}", s)
    }
}

fn parse_result(s: &str) -> Ordering {
    match s {
        "X" => Ordering::Less,
        "Y" => Ordering::Equal,
        "Z" => Ordering::Greater,
        _ => panic!("Unknown input: {}", s)
    }
}
