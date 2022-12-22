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
                (x, y, dir) = forward((x, y), *n, dir, &map);
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

fn forward(pos: (usize, usize), step: usize, dir: Dir, map: &Vec<Vec<char>>) -> (usize, usize, Dir) {
    let (mut x, mut y) = pos;
    let mut step = step;
    let mut dir = dir;
    while step > 0 {
        let (m, n, d) = next_pos(x, y, dir);
        match map[m][n] {
            '#' => {
                break;
            },
            '.' => {
                step -= 1;
                (x, y) = (m, n);
                dir = d;
            },
            _ => {
                panic!()
            },
        }
        println!("{} {} {:?}", x, y, dir);
    }
    (x, y, dir)
}

fn next_pos(x: usize, y: usize, dir: Dir) -> (usize, usize, Dir){
    match dir {
        Dir::U => {
            if x == 100 && y < 50 {
                (50 + y, 50, Dir::R) // a
            } else if x == 0 && y >= 50 && y < 100 {
                (100 + y, 0, Dir::R) // b
            } else if x == 0 && y >= 100 && y < 150 {
                (199, y - 100, Dir::U) // c
            } else {
                (x - 1, y, dir)
            }
        },
        Dir::R => {
            if y == 149 && x < 50{
                (149 - x, 99, Dir::L) // d
            } else if y == 99 && x >= 50 && x < 100 {
                (49, x + 50, Dir::U) // e
            } else if y == 99 && x >= 100 && x < 150 {
                (149 - x, 149, Dir::L) // d
            } else if y == 49 && x >= 150 && x < 200 {
                (149, x - 100, Dir::U) // f
            } else {
                (x, y + 1, dir)
            }
        },
        Dir::D => {
            if x == 199 && y < 50 {
                (0, 100 + y, Dir::D) // c
            } else if x == 149 && y >= 50 && y < 100 {
                (100 + y, 49, Dir::L) // f
            } else if x == 49 && y >= 100 && y < 150 {
                (y - 50, 99, Dir::L) // e
            } else {
                (x + 1, y, dir)
            }
        },
        Dir::L => {
            if y == 0 && x >= 150 && x < 199 {
                (0, x - 100, Dir::D) // b
            } else if y == 0 && x >= 100 && x < 150 {
                (149 - x, 50, Dir::R) // g
            } else if y == 50 && x >= 50 && x < 100 {
                (100, x - 50, Dir::D) // a
            } else if y == 50 && x < 50 {
                (149 - x, 0, Dir::R) // g
            } else {
                (x, y - 1, dir)
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
