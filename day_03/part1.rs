use std::io;


fn main() {
    let mut score = 0;
    for l in io::stdin().lines() {
        let line = l.unwrap();
        let mut bits : u64 = 0;
        for (i, c) in line.chars().enumerate() {
            let chr = c as u8;
            let offset = chr - 'A' as u8;
            let mask = 1 << offset;
            if i < line.len() / 2 {
                bits |= mask
            } else {
                if bits & mask > 0 {
                    score += offset as i32 + if offset < 26 {27} else {-31};
                    break;
                }
            }
        }
    }
    println!("{}", score);
}
