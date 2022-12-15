use std::cmp::max;

// const Y: i64 = 20;
const Y: i64 = 4000000;

fn main() {
    let mut dists = vec![];
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        let (x1, y1, x2, y2) = parse(&line);
        let dist = (x1 - x2).abs() + (y1 - y2).abs();
        dists.push((x1, y1, dist));
    }
    for y in 0..=Y {
        let mut ranges: Vec<(i64, i64)> = vec![];
        for (x1, y1, dist) in dists.iter() {
            let span = dist - (y1 - y).abs();
            if span >= 0 {
                ranges.push((x1 - span, x1 + span));
            }
        }
        ranges.sort();
        let x = find_ranges(&ranges);
        if x >= 0 {
            println!("{}, {}, ans = {}", x, y, x * 4000000 + y);
            return;
        }
    }
}

fn find_ranges(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut i = 1;
    let (_, mut r) = ranges[0];
    while i < ranges.len() {
        let (m, n) = ranges[i];
        if r + 1 > Y {
            break;
        }
        if m <= r || r + 1 < 0 {
            r = max(r, n);
        } else {
          return r + 1;
        }
        i += 1;
    }
    -1
}

fn parse(s: &str) -> (i64, i64, i64, i64) {
    if let [s1, s2] = s.split(": ").collect::<Vec<&str>>()[..] {
        let (x1, y1) = parse_point(s1);
        let (x2, y2) = parse_point(s2);
        (x1, y1, x2, y2)
    } else {
        panic!("bad input: {}", s);
    }
}

fn parse_point(s: &str) -> (i64, i64) {
    let l = s.find("x=").unwrap();
    let r = s.find(',').unwrap();
    let x = s[l+2..r].parse().unwrap();
    let l = s.find("y=").unwrap();
    let y = s[l+2..].parse().unwrap();
    (x, y)
}
