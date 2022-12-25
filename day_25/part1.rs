fn main() {
    let nums = parse();
    let mut ans = 0;
    for n in nums.iter() {
        ans += to_tens(&n);
    }
    println!("ans = {}, {}", ans, to_fives(ans));
    assert!(ans == to_tens(&to_fives(ans)));
}

fn to_tens(s: &str) -> i64 {
    let mut ret = 0;
    for (i, c) in s.chars().rev().enumerate() {
        ret += 5i64.pow(i as u32) * match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        }
    }
    ret
}

fn to_fives(n: i64) -> String {
    let mut s = String::new();
    let mut i = 0;
    let mut m = 2 * 5i64.pow(i);
    while n > m || n < -m {
        i += 1;
        m += 2 * 5i64.pow(i);
    }
    to_fives_helper(n, i, &mut s);
    s
}

fn to_fives_helper(n: i64, i: u32, ret: &mut String){
    match n {
        -2 => {
            ret.push('=');
            return;
        },
        -1 => {
            ret.push('-');
            return;
        },
        0 => {
            ret.push('0');
            return;
        },
        1 => {
            ret.push('1');
            return;
        },
        2 => {
            ret.push('2');
            return;
        },
        _ => {
            let mut m = 0;
            for k in 0..=i {
                m += 2 * 5i64.pow(k);
            }
            let mut p = 2 * 5i64.pow(i);
            let mut j = i;
            while j > 0 {
                j -= 1;
                p -= 2 * 5i64.pow(j); 
            }
            if n >= p {
                ret.push('2');
                to_fives_helper(n - 2 * 5i64.pow(i), i - 1, ret);
            } else if n > m - 2 * 5i64.pow(i) {
                ret.push('1');
                to_fives_helper(n - 1 * 5i64.pow(i), i - 1, ret);
            } else if n <= -p {
                ret.push('=');
                to_fives_helper(n + 2 * 5i64.pow(i), i - 1, ret);
            } else if n < -m + 2 * 5i64.pow(i) {
                ret.push('-');
                to_fives_helper(n + 1 * 5i64.pow(i), i - 1, ret);
            } else {
                ret.push('0');
                to_fives_helper(n, i - 1, ret);
            }
        }
    }
}


fn parse() -> Vec<String> {
    let mut ret = vec![];
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        ret.push(String::from(line));
    }
    ret
}
