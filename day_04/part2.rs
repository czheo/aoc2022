use std::io;


fn main() {
    let mut ans = 0;
    for l in io::stdin().lines() {
        let line = l.unwrap();
        let mut it = line.split(',').take(2);
        let (f1, t1) = parse(it.next().unwrap());
        let (f2, t2) = parse(it.next().unwrap());
        if (f1 <= f2 && t1 >= f2) || (f2 <= f1 && t2 >= f1) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn parse(s: &str) -> (u8, u8) {
    let mut it = s.split('-').take(2);
    let p1 = it.next().unwrap().parse().unwrap();
    let p2 = it.next().unwrap().parse().unwrap();
    (p1, p2)
}
