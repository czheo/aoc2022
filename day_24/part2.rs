use std::collections::{VecDeque, HashSet};

fn main() {
    let (mut left, mut right, mut up, mut down) = parse();
    let mut map = vec![0;left.len()];
    let t1 = run(0, 0, left.len() + 1, up.len() - 1, &mut left, &mut right, &mut up, &mut down, &mut map);
    let t2 = run(left.len() + 1, up.len() - 1, 0, 0, &mut left, &mut right, &mut up, &mut down, &mut map) + 1;
    let t3 = run(0, 0, left.len() + 1, up.len() - 1, &mut left, &mut right, &mut up, &mut down, &mut map) + 1;
    println!("{} {} {}, ans = {}", t1, t2, t3, t1 + t2 + t3);
}

fn run(x: usize, y: usize, to_x: usize, to_y: usize,
    left: &mut Vec<u128>,
    right: &mut Vec<u128>,
    up: &mut Vec<u128>,
    down: &mut Vec<u128>,
    map: &mut Vec<u128>)  -> usize {
    let mut t = 0;
    let mut queue = VecDeque::from([(x, y)]);
    let mut seen = HashSet::from([(x, y, hash(&left, &right, &up, &down))]);
    loop {
        // _print_map(&map, up.len());
        next_tick(left, right, up, down, map);
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();
            // println!("{}, {} | t = {}", x, y, t);
            if (x, y) == (to_x, to_y) {
                return t;
            }
            let map_hash = hash(&left, &right, &up, &down);
            let mut next_p = vec![(x, y)];
            if (x, y) == (0, 0) {
                next_p.push((x + 1, y));
            } else if (x, y) == (1, 0) {
                next_p.push((x - 1, y));
                next_p.push((x + 1, y));
                next_p.push((x, y + 1));
            } else if (x, y) == (left.len() + 1, up.len() - 1) {
                next_p.push((x - 1, y));
            } else if (x, y) == (left.len(), up.len() - 1) {
                next_p.push((x + 1, y));
                next_p.push((x - 1, y));
                next_p.push((x, y - 1));
            } else {
                if x != left.len() {
                    next_p.push((x + 1, y));
                }
                if y != up.len() - 1 {
                    next_p.push((x, y + 1));
                }
                if x != 1 {
                    next_p.push((x - 1, y));
                }
                if y != 0 {
                    next_p.push((x, y - 1));
                }
            }
            for (x, y) in next_p {
                if is_valid(x, y, &map, up.len()) && !seen.contains(&(x, y, map_hash.clone())) {
                    seen.insert((x, y, map_hash.clone()));
                    queue.push_back((x, y));
                }
            }
        }
        t += 1;
    }
}

fn hash(left: &Vec<u128>, right: &Vec<u128>, up: &Vec<u128>, down: &Vec<u128>) -> String {
    let mut s = String::new();
    for n in left.iter() {
        s.push_str(&n.to_string());
        s.push(',');
    }
    for n in right.iter() {
        s.push_str(&n.to_string());
        s.push(',');
    }
    for n in up.iter() {
        s.push_str(&n.to_string());
        s.push(',');
    }
    for n in down.iter() {
        s.push_str(&n.to_string());
        s.push(',');
    }
    s
}

fn is_valid(x: usize, y: usize, map: &Vec<u128>, width: usize) -> bool {
    (x, y) == (0, 0)
        || (x, y) == (map.len() + 1, width - 1)
        || map[x - 1] & (1 << y) == 0
}

fn next_tick(left: &mut Vec<u128>, right: &mut Vec<u128>, up: &mut Vec<u128>, down: &mut Vec<u128>, map: &mut Vec<u128>) {
    next_pos(left, right, up, down);
    compute_map(&left, &right, &up, &down, map);
}

fn next_pos(left: &mut Vec<u128>, right: &mut Vec<u128>, up: &mut Vec<u128>, down: &mut Vec<u128>) {
    let h = left.len();
    let w = up.len();
    for n in left {
        if *n & 1 > 0 {
            *n |= 1 << w;
        }
        *n >>= 1;
    }
    for n in right {
        *n <<= 1;
        if *n & 1 << w > 0 {
            *n ^= 1<<w;
            *n |= 1;
        }
    }
    for n in up {
        if *n & 1 > 0 {
            *n |= 1 << h;
        }
        *n >>= 1;
    }
    for n in down {
        *n <<= 1;
        if *n & 1 << h > 0 {
            *n ^= 1<<h;
            *n |= 1;
        }
    }
}

fn compute_map(left: &Vec<u128>, right: &Vec<u128>, up: &Vec<u128>, down: &Vec<u128>, map: &mut Vec<u128>) {
    for i in 0..left.len() {
        map[i] = left[i];
        map[i] |= right[i];
        for j in 1..up.len() - 1 {
            if up[j] & 1 << i > 0 {
                map[i] |= 1 << j;
            }
            if down[j] & 1 << i > 0 {
                map[i] |= 1 << j;
            };
        }
    }
}

fn _print_map(map: &Vec<u128>, width: usize) {
    println!();
    for n in map.iter() {
        for i in 0..width {
            if n & 1 << i > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse() -> (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>){
    let mut it = std::io::stdin().lines();
    it.next();
    let mut map: Vec<Vec<char>> = vec![];
    while let Some(Ok(line)) = it.next() {
        if line.starts_with("##") {
            break;
        }
        map.push(line[1..line.len() - 1].chars().collect());
    }
    let mut left = vec![];
    let mut right = vec![];
    for line in map.iter() {
        let mut l: u128 = 0;
        let mut r: u128 = 0;
        for (i, c) in line.iter().enumerate() {
            if *c == '<' {
                l |= 1 << i;
            } else if *c == '>' {
                r |= 1 << i;
            }
        }
        left.push(l);
        right.push(r);
    }
    let mut up = vec![];
    let mut down = vec![];
    for j in 0..map[0].len() {
        let mut i = 0;
        let mut u: u128 = 0;
        let mut d: u128 = 0;
        while i < map.len() {
            if map[i][j] == '^' {
                u |= 1 << i;
            } else if map[i][j] == 'v' {
                d |= 1 << i;
            }
            i += 1;
        }
        up.push(u);
        down.push(d);
    }
    (left, right, up, down)
}
