fn main() {
    let mut input = parse();
    let l = input.len();
    let mut index: Vec<usize> = (0..l).collect(); // track original index -> current index

    decrypt(&mut input, &mut index);

    let zero = input.iter().position(|(_, x)| *x == 0).unwrap();
    let (_, a) = input[(zero + 1000) % l];
    let (_, b) = input[(zero + 2000) % l];
    let (_, c) = input[(zero + 3000) % l];
    println!("{} {} {}, ans = {}", a, b, c, a + b + c)
}

fn decrypt(input: &mut Vec<(usize, i32)>, index: &mut Vec<usize>) {
    let l = input.len();
    let mut i = 0;
    while i < l {
        let j = index[i];
        let (_, n) = input[j];
        let p = next_position(n, j, l);
        shift(j, p, input, index);
        i += 1;
    }
}

/*
 * The next position follow a pattern like below.
 * when number is positive:
 * 0 1 2 3
 *   4 5 6
 *   7 8 9
 * 
 * when negative:
 *   2 1 0
 *   5 4 3
 *   8 7 6
 */
fn next_position(n: i32, j: usize, l: usize)-> usize {
    if n == 0 {
        j
    } else if n > 0 {
        (j + n as usize - 1) % (l - 1) + 1
    } else {
        l - (l - j + (-n) as usize - 1) % (l - 1) - 1
    }
}

fn shift(j: usize, p: usize, input: &mut Vec<(usize, i32)>, index: &mut Vec<usize>) {
    let (x, m) = input[j];
    let mut j = j;
    while j != p {
        let (y, n) = if j < p {
            input[j + 1]
        } else {
            input[j - 1]
        };
        input[j] = (y, n);
        index[y] = j;
        if j < p {
            j += 1;
        } else {
            j -= 1;
        }
    }
    input[p] = (x, m);
    index[x] = p;
}

fn parse() -> Vec<(usize, i32)> {
    std::io::stdin().lines()
        .map(|x| x.unwrap().parse().unwrap())
        .enumerate()
        .collect()
}
