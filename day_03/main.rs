use std::io;


fn main() {
    let mut score = 0;
    let mut all_bits : u64 = !0;
    for (i, l) in io::stdin().lines().enumerate() {
        let line = l.unwrap();
        let mut bits : u64 = 0;
        for c in line.chars() {
            let chr = c as u8;
            let offset = chr - 'A' as u8;
            bits |= 1 << offset; 
        }
        all_bits &= bits;
        if (i + 1) % 3 == 0 {
            let offset = get_offset(all_bits);
            score += offset + if offset < 26 {27} else {-31};
            all_bits = !0;
        }
    }
    println!("{}", score);
}

fn get_offset(bits: u64) -> i32 {
    for i in 0..=63 {
        let mask = 1 << i;
        if mask == bits {
            return i;
        }
    }
    return 0; // should never happen
}
