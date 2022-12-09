use std::collections::HashSet;

fn main() {
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];
    let mut set = HashSet::new();
    set.insert((0, 0));
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        let ((dx, dy), step) = parse(&line);
        for _ in 0..step {
            let (x, y) = rope[0];
            rope[0] = (x + dx, y + dy);
            for i in 1..10 {
                rope[i] = next_pos(rope[i-1], rope[i]);
            }
            set.insert(rope[9]);
        }
    }

    println!("{}", set.len());
}

fn next_pos(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (hx, hy) = head;
    let (tx, ty) = tail;
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
