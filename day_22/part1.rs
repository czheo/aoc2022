use std::cmp::max;

#[derive(Debug)]
enum Inst {
    L,
    R,
    F(usize),
}

#[derive(Debug,Clone,Copy)]
enum Dir {
    R = 0,
    D = 1,
    L = 2,
    U = 3,
}

fn main() {
    let (map, insts) = parse();
    let mut i = 0;
    while map[0][i] == ' ' {
        i += 1;
    }
    let (mut x, mut y) = (0, i);
    let mut dir = Dir::R;
    for inst in insts.iter() {
        match inst {
            Inst::L => {
                dir = turn_l(dir);
            },
            Inst::R => {
                dir = turn_r(dir);
            },
            Inst::F(n) => {
                (x, y) = forward((x, y), *n, dir, &map);
            },
        }
    }

    println!("{:?} {:?}, ans = {}", (x, y), dir, (x + 1) * 1000 + (y + 1) * 4 + dir as usize);
}

fn turn_l(dir: Dir) -> Dir{
    match dir {
        Dir::U => Dir::L,
        Dir::L => Dir::D,
        Dir::D => Dir::R,
        Dir::R => Dir::U,
    }
}

fn turn_r(dir: Dir) -> Dir{
    match dir {
        Dir::U => Dir::R,
        Dir::R => Dir::D,
        Dir::D => Dir::L,
        Dir::L => Dir::U,
    }
}

fn forward(pos: (usize, usize), step: usize, dir: Dir, map: &Vec<Vec<char>>) -> (usize, usize) {
    let (mut x, mut y) = pos;
    let (max_x, max_y) = (map.len(), map[0].len());
    let mut step = step;
    while step > 0 {
        let (mut m, mut n) = next_pos(x, y, max_x, max_y, dir);
        while map[m][n] == ' ' {
            (m, n) = next_pos(m, n, max_x, max_y, dir);
        }
        match map[m][n] {
            '#' => {
                break;
            },
            '.' => {
                step -= 1;
                (x, y) = (m, n);
            },
            _ => {
            },
        }
    }
    (x, y)
}

fn next_pos(x: usize, y: usize, max_x: usize, max_y: usize, dir: Dir) -> (usize, usize){
    match dir {
        Dir::U => {
            if x == 0 {
                (max_x - 1, y)
            } else {
                (x - 1, y)
            }
        },
        Dir::R => {
            if y == max_y - 1 {
                (x, 0)
            } else {
                (x, y + 1)
            }
        },
        Dir::D => {
            if x == max_x - 1 {
                (0, y)
            } else {
                (x + 1, y)
            }
        },
        Dir::L => {
            if y == 0 {
                (x, max_y - 1)
            } else {
                (x, y - 1)
            }
        },
    }
}

fn parse() -> (Vec<Vec<char>>, Vec<Inst>) {
    let mut it = std::io::stdin().lines();
    let mut map:Vec<Vec<char>> = vec![];
    let mut max_r = 0;
    while let Some(Ok(line)) = it.next() {
        max_r = max(max_r, line.len());
        map.push(line.chars().collect());
        if line == "" {
            break;
        }
    }

    let mut i = 0;
    while i < map.len() {
        while map[i].len() < max_r {
            map[i].push(' ');
        }
        i += 1;
    }

    if let Some(Ok(line)) = it.next() {
        return (map, parse_insts(&line));
    } else {
        panic!();
    }
}

fn parse_insts(s: &str) -> Vec<Inst>{
    let s = s.replace("R", " R ");
    let s = s.replace("L", " L ");
    s.split(' ').map(|x| match x {
        "L" => Inst::L,
        "R" => Inst::R,
        n => Inst::F(n.parse().unwrap()),
    }).collect()
}
