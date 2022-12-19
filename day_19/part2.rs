use std::cmp::max;
use std::collections::HashMap;

type Blueprint = (i32, i32, i32, i32, i32, i32);
type State = (i32, i32, i32, i32, i32, i32, i32, i32);

fn main() {
    let blueprints = parse();
    let mut ans = 1;
    for (i, blueprint) in blueprints.iter().enumerate() {
        if i > 2 {
            break;
        }
        let s = solve(blueprint);
        ans *= s;
        println!("{:?} {}", blueprint, s);
    }
    println!("ans = {}", ans);
}

fn solve(blueprint: &Blueprint) -> i32 {
    let mut cache = HashMap::new();
    dp(32, (0, 0, 0, 0, 1, 0, 0, 0), blueprint, &mut cache)
}

fn dp(t: i32, state: State, blueprint: &Blueprint,
    cache: &mut HashMap<(i32, State), i32>) -> i32 {
    let (mut a, mut b, mut c, d, mut ma, mut mb, mut mc, md) = state;
    if t <= 0 {
        return d;
    }
    let (aa, ba, ca, cb, da, dc) = *blueprint;
    let max_a = [aa, ba, ca, da].into_iter().max().unwrap();
    // Prune the search space:
    // Intuition: no need to produce more machines / resources if already enough
    // 1. # of machine A is enough to build any other machine
    if ma > max_a {
        ma = max_a;
    }
    if mb > cb {
        mb = cb;
    }
    if mc > dc {
        mc = dc;
    }
    // 2. # of resource A is enough to build any other machine in this time frame
    // and if no more machine A will be created, we need to consume (max_a - ma) resource A in each
    // future time frame
    if a > max_a + (t - 1) * (max_a - ma) {
        a = max_a + (t - 1) * (max_a - ma);
    }
    if b > cb + (t - 1) * (cb - mb) {
        b = cb + (t - 1) * (cb - mb);
    }
    if c > dc + (t - 1) * (dc - mc) {
        c = dc + (t - 1) * (dc - mc);
    }
    let state = (a, b, c, d, ma, mb, mc, md);
    let key = (t, state);
    if cache.contains_key(&key) {
        return cache[&key]
    }
    if a >= da && c >= dc {
        let ret = dp(t - 1,
            (a + ma - da, b + mb, c + mc - dc, d + md, ma, mb, mc, md + 1),
            blueprint, cache);
        cache.insert(key, ret);
        return ret;
    }
    let mut ret = dp(t - 1,
        (a + ma, b + mb, c + mc , d + md, ma, mb, mc, md),
        blueprint, cache);
    if a >= aa {
        ret = max(ret, dp(t - 1,
            (a + ma - aa, b + mb, c + mc, d + md, ma + 1, mb, mc, md),
            blueprint, cache));
    }
    if a >= ba {
        ret = max(ret, dp(t - 1,
            (a + ma - ba, b + mb, c + mc, d + md, ma, mb + 1, mc, md),
            blueprint, cache));
    }
    if a >= ca && b >= cb {
        ret = max(ret, dp(t - 1,
            (a + ma - ca, b + mb - cb, c + mc, d + md, ma, mb, mc + 1, md),
            blueprint, cache));
    }
    cache.insert(key, ret);
    ret
}

fn parse() -> Vec<Blueprint> {
    let mut ret = vec![];
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        let i = line.find(":").unwrap();
        let mut it = line[i+2..].split(". ");
        let l = it.next().unwrap();
        let a = l.split(' ').collect::<Vec<&str>>()[4].parse().unwrap();
        let l = it.next().unwrap();
        let b = l.split(' ').collect::<Vec<&str>>()[4].parse().unwrap();
        let l = it.next().unwrap();
        let v = l.split(' ').collect::<Vec<&str>>();
        let c1 = v[4].parse().unwrap();
        let c2 = v[7].parse().unwrap();
        let l = it.next().unwrap();
        let v = l.split(' ').collect::<Vec<&str>>();
        let d1 = v[4].parse().unwrap();
        let d2 = v[7].parse().unwrap();
        ret.push((a, b, c1, c2, d1, d2));
    }
    ret
}
