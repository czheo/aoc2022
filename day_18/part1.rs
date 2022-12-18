use std::collections::HashSet;

fn main() {
    let points = parse();
    let mut ans = 0;
    for (x, y, z) in points.iter() {
        if !points.contains(&(x + 1, *y, *z)) {
            ans += 1
        }
        if !points.contains(&(x - 1, *y, *z)) {
            ans += 1
        }
        if !points.contains(&(*x, y + 1, *z)) {
            ans += 1
        }
        if !points.contains(&(*x, y - 1, *z)) {
            ans += 1
        }
        if !points.contains(&(*x, *y, z + 1)) {
            ans += 1
        }
        if !points.contains(&(*x, *y, z - 1)) {
            ans += 1
        }
    }
    println!("{}", ans);
}

fn parse() -> HashSet<(i8, i8, i8)> {
    let mut ret = HashSet::new();
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        if let [x, y, z] = line.split(',')
            .map(|x| x.parse().unwrap())
                .collect::<Vec<i8>>()[..] {
            ret.insert((x, y, z));
        }
    }
    ret
}
