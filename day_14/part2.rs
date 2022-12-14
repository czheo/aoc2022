use std::collections::HashSet;
use std::cmp::{max, min};

fn main() {
    let (mut map, max_y) = parse();
    let ans = simulate(&mut map, max_y + 1);
    println!("{}", ans);
}

fn simulate(map: &mut HashSet<(i32, i32)>, max_y: i32) -> usize{
    let mut ans = 0;
    loop {
        let (x, y) = one_sand(&map, max_y);
        map.insert((x, y));
        ans += 1;
        if (x, y) == (500, 0) {
            return ans;
        }
    }
}

fn one_sand(map: &HashSet<(i32, i32)>, max_y: i32) -> (i32, i32) {
    let mut x = 500;
    let mut y = 0;
    loop {
        let (i, j) = next_tick(&map, x, y);
        if (i, j) == (x, y) || j >= max_y {
            return (i, j);
        } else {
            x = i;
            y = j;
        }
    }
}

fn next_tick(map: &HashSet<(i32, i32)>, x: i32, y: i32) -> (i32, i32) {
    if !map.contains(&(x, y + 1)) {
        (x, y + 1)
    } else if !map.contains(&(x - 1, y + 1)) {
        (x - 1, y + 1)
    } else if !map.contains(&(x + 1, y + 1)) {
        (x + 1, y + 1)
    } else {
        (x, y)
    }
}

fn parse() -> (HashSet<(i32, i32)>, i32) {
    let mut map = HashSet::new();
    let mut max_y = 0;
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        max_y = max(parse_path(&line, &mut map), max_y);
    }
    (map, max_y)
}

fn parse_path(s: &str, map: &mut HashSet<(i32, i32)>) -> i32 {
    let points: Vec<(i32, i32)> = s.split(" -> ").map(|x| parse_point(x)).collect();
    let mut max_y = 0;
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        for x in min(x1, x2)..=max(x1, x2) {
            map.insert((x, y1));
        }
        for y in min(y1, y2)..=max(y1, y2) {
            map.insert((x1, y));
            max_y = max(y, max_y);
        }
    }
    max_y
}

fn parse_point(s: &str) -> (i32, i32) {
    if let [x, y] = s.split(',').map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>()[..] {
        (x, y)
    } else {
        panic!("bad point input: {}", s)
    }
}
