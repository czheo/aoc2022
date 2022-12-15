use std::collections::HashSet;
use std::cmp::max;

// const Y: i32 = 10;
const Y: i32 = 2000000;

fn main() {
    let mut ranges: Vec<(i32, i32)> = vec![];
    let mut at_line = HashSet::new();
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        let (x1, y1, x2, y2) = parse(&line);
        let dist = (x1 - x2).abs() + (y1 - y2).abs();
        let span = dist - (y1 - Y).abs();
        if y2 == Y {
            at_line.insert(x2);
        }
        if span >= 0 {
            ranges.push((x1 - span, x1 + span));
        }
    }
    ranges.sort();
    println!("{}", count_ranges(&ranges) - at_line.len() as i32);
}

fn count_ranges(ranges: &Vec<(i32, i32)>) -> i32 {
    let mut i = 1;
    let (mut l, mut r) = ranges[0];
    let mut ret = 0;
    while i < ranges.len() {
        let (m, n) = ranges[i];
        if m <= r {
            r = max(n, r);
        } else {
            ret += r - l + 1;
            (l, r) = (m, n);
        }
        i += 1;
    }
    ret + (r - l + 1)
}

fn parse(s: &str) -> (i32, i32, i32, i32) {
    if let [s1, s2] = s.split(": ").collect::<Vec<&str>>()[..] {
        let (x1, y1) = parse_point(s1);
        let (x2, y2) = parse_point(s2);
        (x1, y1, x2, y2)
    } else {
        panic!("bad input: {}", s);
    }
}

fn parse_point(s: &str) -> (i32, i32) {
    let l = s.find("x=").unwrap();
    let r = s.find(',').unwrap();
    let x = s[l+2..r].parse().unwrap();
    let l = s.find("y=").unwrap();
    let y = s[l+2..].parse().unwrap();
    (x, y)
}
