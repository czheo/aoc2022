use std::collections::{HashSet, HashMap};
use std::cmp::{max, min};

fn main() {
    let dirs = ["N", "S", "W", "E"];
    let mut positions = parse();
    for t in 0..10 {
        let proposal = get_proposal(&positions, t, &dirs);
        for ((x, y), vec) in proposal.iter() {
            if vec.len() == 1 {
                let i = vec[0];
                positions[i] = (*x, *y);
            }
        }
        print_map(&positions);
    }
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    for (x, y) in positions.iter() {
        max_x = max(max_x, *x);
        max_y = max(max_y, *y);
        min_x = min(min_x, *x);
        min_y = min(min_y, *y);
    }
    println!("ans = {}", (max_x - min_x + 1) * (max_y - min_y + 1) - positions.len() as i64);
}

fn print_map(positions: &Vec<(i64, i64)>) {
    let pos_set: HashSet<(i64, i64)> = HashSet::from_iter(positions.clone());
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    for (x, y) in positions.iter() {
        max_x = max(max_x, *x);
        max_y = max(max_y, *y);
        min_x = min(min_x, *x);
        min_y = min(min_y, *y);
    }
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if pos_set.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn get_proposal(positions: &Vec<(i64, i64)>,
    t: usize,
    dirs: &[&str]) -> HashMap<(i64, i64), Vec<usize>> {
    let pos_set: HashSet<(i64, i64)> = HashSet::from_iter(positions.clone());
    let mut proposal: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (i, (x, y)) in positions.iter().enumerate() {
        if !(pos_set.contains(&(x - 1, y - 1))
            || pos_set.contains(&(x - 1, y + 1))
            || pos_set.contains(&(x + 1, y - 1))
            || pos_set.contains(&(x + 1, y + 1))
            || pos_set.contains(&(x - 1, *y))
            || pos_set.contains(&(x + 1, *y))
            || pos_set.contains(&(*x, y + 1))
            || pos_set.contains(&(*x, y - 1))) {
            continue;
        }
        for j in 0..4 {
            match dirs[(t + j) % 4] {
                "N" => {
                    if !(pos_set.contains(&(x - 1, y - 1))
                        || pos_set.contains(&(x - 1, *y))
                        || pos_set.contains(&(x - 1, y + 1))) {
                        if let Some(v) = proposal.get_mut(&(x - 1, *y)) {
                            v.push(i);
                        } else {
                            proposal.insert((x - 1, *y), vec![i]);
                        }
                        break;
                    }
                }
                "S" => {
                    if !(pos_set.contains(&(x + 1, y - 1))
                        || pos_set.contains(&(x + 1, *y))
                        || pos_set.contains(&(x + 1, y + 1))) {
                        if let Some(v) = proposal.get_mut(&(x + 1, *y)) {
                            v.push(i);
                        } else {
                            proposal.insert((x + 1, *y), vec![i]);
                        }
                        break;
                    }
                },
                "W" => {
                    if !(pos_set.contains(&(x - 1, y - 1))
                        || pos_set.contains(&(*x, y - 1))
                        || pos_set.contains(&(x + 1, y - 1))) {
                        if let Some(v) = proposal.get_mut(&(*x, y - 1)) {
                            v.push(i);
                        } else {
                            proposal.insert((*x, y - 1), vec![i]);
                        }
                        break;
                    }
                },
                "E" => {
                    if !(pos_set.contains(&(x - 1, y + 1))
                        || pos_set.contains(&(*x, y + 1))
                        || pos_set.contains(&(x + 1, y + 1))) {
                        if let Some(v) = proposal.get_mut(&(*x, y + 1)) {
                            v.push(i);
                        } else {
                            proposal.insert((*x, y + 1), vec![i]);
                        }
                        break;
                    }
                },
                _ => panic!(),
            }
        }
    }
    proposal
}

fn parse() -> Vec<(i64, i64)> {
    let mut ret = vec![];
    let mut i = 0;
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        let mut j = 0;
        for c in line.chars() {
            if c == '#' {
                ret.push((i, j));
            }
            j += 1;
        }
        i += 1;
    }
    ret
}
