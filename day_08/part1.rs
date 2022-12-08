fn main() {
    let (input, mut visibility) = init();
    let (m, n) = (input.len(), input[0].len());
    for i in 0..m {
        let mut max = -1;
        for j in (0..n).rev() {
            if input[i][j] > max {
                visibility[i][j] = true;
            }
            max = std::cmp::max(max, input[i][j]);
        }
    }
    for j in 0..n {
        let mut max = -1;
        for i in 0..m {
            if input[i][j] > max {
                visibility[i][j] = true;
            }
            max = std::cmp::max(max, input[i][j]);
        }
        max = -1;
        for i in (0..m).rev() {
            if input[i][j] > max {
                visibility[i][j] = true;
            }
            max = std::cmp::max(max, input[i][j]);
        }
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if visibility[i][j] == true {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn init() -> (Vec<Vec<i8>>, Vec<Vec<bool>>) {
    let mut input: Vec<Vec<i8>> = Vec::new();
    let mut visibility: Vec<Vec<bool>> = Vec::new();
    for l in std::io::stdin().lines() {
        let mut ri = Vec::new();
        let mut rv = Vec::new();
        let mut max: i8 = -1;
        for c in l.unwrap().chars() {
            let n = c.to_digit(10).unwrap() as i8;
            ri.push(n);
            rv.push(n > max);
            max = std::cmp::max(max, n);
        }
        input.push(ri);
        visibility.push(rv);
    }
    (input, visibility)
}

