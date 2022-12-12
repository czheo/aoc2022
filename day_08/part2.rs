fn main() {
    let input = init();
    let (m, n) = (input.len(), input[0].len());
    let mut max = 0;
    for i in 1..m-1 {
        for j in 1..n-1 {
            max = std::cmp::max(max, score(&input, i, j, m, n));
        }
    }
    println!("{}", max);
}

fn score(input: &Vec<Vec<u32>>, i: usize, j: usize, m: usize, n: usize) -> usize{
    let mut score = 1;
    let mut x = i + 1;
    while x < m - 1 {
        if input[x][j] >= input[i][j] {
            break;
        }
        x += 1;
    }
    score *= x - i;
    x = i - 1;
    while x > 0 {
        if input[x][j] >= input[i][j] {
            break;
        }
        x -= 1;
    }
    score *= i - x;
    x = j + 1;
    while x < n - 1{
        if input[i][x] >= input[i][j] {
            break;
        }
        x += 1;
    }
    score *= x - j;
    x = j - 1;
    while x > 0 {
        if input[i][x] >= input[i][j] {
            break;
        }
        x -= 1;
    }
    score *= j - x;
    score
}

fn init() -> Vec<Vec<u32>> {
    std::io::stdin().lines()
        .map(|l|
            l.unwrap().chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect())
        .collect()
}

