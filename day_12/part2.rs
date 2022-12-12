use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

fn main() {
    let (map, (ex, ey)) = parse();
    let (m, n) = (map.len(), map[0].len());
    let mut queue = BinaryHeap::from([Reverse((0, ex, ey))]);
    let mut visited = HashSet::from([(ex, ey)]);
    let mut ret = 0;
    while let Some(Reverse((dist, x, y))) = queue.pop() {
        if map[x][y] == 0 {
            ret = dist;
            break;
        }
        for (i, j) in neighbors(x, y, m, n, &map) {
            if !visited.contains(&(i, j)) {
                visited.insert((i, j));
                queue.push(Reverse((dist + 1, i, j)));
            }
        }
    }
    println!("{}", ret);
}

fn neighbors(x: usize, y: usize, m: usize, n: usize, map: &Vec<Vec<i8>>)
    -> Vec<(usize, usize)> {
    let mut v = vec![];
    if x != 0 {
        if (map[x - 1][y] - map[x][y]) >= -1 {
            v.push((x - 1, y));
        }
    }
    if y != 0 {
        if (map[x][y - 1] - map[x][y]) >= -1 {
            v.push((x, y - 1));
        }
    }
    if x < m - 1 {
        if (map[x + 1][y] - map[x][y]) >= -1 {
            v.push((x + 1, y));
        }
    }
    if y < n - 1 {
        if (map[x][y + 1] - map[x][y]) >= -1 {
            v.push((x, y + 1));
        }
    }
    v
}

fn parse() -> (Vec<Vec<i8>>, (usize, usize)){
    let mut end = (0, 0);
    let mut map = Vec::new();
    for (i, l) in std::io::stdin().lines().enumerate() {
        let mut v = vec![];
        for (j, c) in l.unwrap().chars().enumerate() {
            let height = if c == 'S' {
                0
            } else if c == 'E' {
                end = (i, j);
                'z' as i8 - 'a' as i8
            } else {
                c as i8 - 'a' as i8
            };
            v.push(height);
        }
        map.push(v);
    }
    (map, end)
}
