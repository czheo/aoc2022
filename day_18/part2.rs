use std::cmp::{min, max};
use std::collections::HashSet;

#[derive(Debug)]
struct Ctx {
    min_x: i8,
    min_y: i8,
    min_z: i8,
    max_x: i8,
    max_y: i8,
    max_z: i8,
}

type Point = (i8, i8, i8);

static DIRS: [Point;6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

impl Ctx {
    fn new() -> Ctx {
        Ctx {
            min_x: i8::MAX,
            min_y: i8::MAX,
            min_z: i8::MAX,
            max_x: i8::MIN,
            max_y: i8::MIN,
            max_z: i8::MIN,
        }
    }
}

fn main() {
    let points = parse();
    let mut ctx = Ctx::new();
    // find boundary
    for (x, y, z) in points.iter() {
        ctx.min_x = min(ctx.min_x, *x);
        ctx.min_y = min(ctx.min_y, *y);
        ctx.min_z = min(ctx.min_z, *z);
        ctx.max_x = max(ctx.max_x, *x);
        ctx.max_y = max(ctx.max_y, *y);
        ctx.max_z = max(ctx.max_z, *z);
    }
    // find trapped air pockets by dfs
    let mut trap = HashSet::new();
    let mut done = HashSet::new();
    for (x, y, z) in points.iter() {
        for (dx, dy, dz) in DIRS {
            let p = (x + dx, y + dy, z + dz);
            if !points.contains(&p) && !done.contains(&p) {
                let mut seen = HashSet::from([p]);
                if let Some(s) = dfs(p, &mut seen, &points, &ctx) {
                    trap.extend(s);
                }
                done.extend(seen);
            }
        }
    }
    // count surface area
    let mut ans = 0;
    for (x, y, z) in points.iter() {
        for (dx, dy, dz) in DIRS {
            let p = (x + dx, y + dy, z + dz);
            if !points.contains(&p) && !trap.contains(&p) {
                ans += 1;
            }
        }
    }
    println!("ans = {}", ans);
}

fn dfs(p: Point, seen: &mut HashSet<Point>,
    points: &HashSet<Point>, ctx: &Ctx)
    -> Option<HashSet<Point>>
{
    let (x, y, z) = p;
    if x <= ctx.min_x || x >= ctx.max_x
    || y <= ctx.min_y || y >= ctx.max_y
    || z <= ctx.min_z || z >= ctx.max_z {
        return None; // if out of boundary, this is not a trapped air pocket.
    }
    let mut ret = HashSet::from([p]);
    for (dx, dy, dz) in DIRS {
        let n = (x + dx, y + dy, z + dz);
        if !points.contains(&n) && !seen.contains(&n) {
            seen.insert(n);
            match dfs(n, seen, &points, &ctx) {
                Some(s) => {
                    ret.extend(s);
                },
                None => return None,
            };
        }
    }
    Some(ret) // return all connected cubes, which belongs to the trapped air pocket.
}

fn parse() -> HashSet<Point> {
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
