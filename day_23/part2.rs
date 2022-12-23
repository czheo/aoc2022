use std::collections::{HashSet, HashMap};

fn main() {
    let dirs = ["N", "S", "W", "E"];
    let mut positions = parse();
    let mut t = 0;
    loop {
        let proposal = get_proposal(&positions, t, &dirs);
        let mut cont = false;
        for ((x, y), vec) in proposal.iter() {
            if vec.len() == 1 {
                cont = true;
                let i = vec[0];
                positions[i] = (*x, *y);
            }
        }
        t += 1;
        if !cont {
            break;
        }
    }
    println!("ans = {}", t);
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
