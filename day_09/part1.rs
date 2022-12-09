use std::collections::HashSet;

fn main() {
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;
    let mut set = HashSet::new();
    set.insert((0, 0));
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        let ((dx, dy), step) = parse(&line);
        for _ in 0..step {
            hx += dx;
            hy += dy;
            (tx, ty) = next_pos(hx, hy, tx, ty);
            set.insert((tx, ty));
        }
    }

    println!("{}", set.len());
}

fn next_pos(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        (tx, ty)
    } else {
        (tx + hx.cmp(&tx) as i32,
         ty + hy.cmp(&ty) as i32)
    }
}

fn parse(s: &str) -> ((i32, i32), i32) {
    let direction = to_direct(s.chars().next().unwrap());
    let step: i32 = s[2..].parse().unwrap();
    (direction, step)
}

fn to_direct(c: char) -> (i32, i32) {
    match c {
        'R' => (1, 0),
        'U' => (0, 1),
        'L' => (-1, 0),
        'D' => (0, -1),
        _ => panic!("Unknown direction {}", c),
    }
}
