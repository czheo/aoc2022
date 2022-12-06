use std::io;
use std::collections::VecDeque;

const L: usize = 4;

fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let mut dq = VecDeque::new();
    let mut ret = 0;
    for c in line.chars() {
        if dq.contains(&c) {
            while !dq.is_empty() {
                let c_ = dq.pop_front().unwrap();
                ret += 1;
                if c_ == c {
                    break
                }
            }
        }

        dq.push_back(c);

        if dq.len() == L {
            println!("{}", ret + L);
            break;
        }
    }
}
