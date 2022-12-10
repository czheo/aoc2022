fn main() {
    let mut ans: i32 = 0;
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut key_cycle: i32 = 20;
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        if line == "noop" {
            cycle += 1;
            if cycle >= key_cycle {
                ans += x * key_cycle;
                key_cycle += 40;
            }
        } else {
            cycle += 2;
            if cycle >= key_cycle {
                ans += x * key_cycle;
                key_cycle += 40;
            }
            let n: i32 = line[5..].parse().unwrap();
            x += n;
        }
    }
    println!("{}", ans);
}
