fn main() {
    let mut x: i32 = 1;
    let mut i: i32 = 0;
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        i = next_tick(x, i);
        if line != "noop" {
            i = next_tick(x, i);
            let n: i32 = line[5..].parse().unwrap();
            x += n;
        }
    }
}

fn next_tick(x: i32, i: i32) -> i32 {
    if i <= x + 1 && i >= x - 1 {
        print!("#");
    } else {
        print!(".");
    }
    let j = (i + 1) % 40;
    if j == 0 {
        println!();
    }
    j
}
