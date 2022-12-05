use std::io;


/*
[N]     [C]                 [Q]
[W]     [J] [L]             [J] [V]
[F]     [N] [D]     [L]     [S] [W]
[R] [S] [F] [G]     [R]     [V] [Z]
[Z] [G] [Q] [C]     [W] [C] [F] [G]
[S] [Q] [V] [P] [S] [F] [D] [R] [S]
[M] [P] [R] [Z] [P] [D] [N] [N] [M]
[D] [W] [W] [F] [T] [H] [Z] [W] [R]
 1   2   3   4   5   6   7   8   9
*/

fn main() {
    let mut stacks = vec![
        vec!['D','M','S','Z','R','F','W','N'],
        vec!['W','P','Q','G','S'],
        vec!['W','R','V','Q','F','N','J','C'],
        vec!['F','Z','P','C','G','D','L'],
        vec!['T','P','S'],
        vec!['H','D','F','W','R','L'],
        vec!['Z','N','D','C'],
        vec!['W','N','R','F','V','S','J','Q'],
        vec!['R','M','S','G','Z','W','V'],
    ];

    let mut it = io::stdin().lines()
        .skip_while(|l| !l.as_ref().unwrap().is_empty());
    it.next();
    for l in it {
        let line = l.unwrap();
        let (mut n, from, to) = parse(&line);
        while n > 0 {
            let x = stacks[from].pop().unwrap();
            stacks[to].push(x);
            n -= 1;
        }
    }

    for v in stacks.iter() {
        print!("{}", v.last().unwrap());
    }
}

fn parse(s: &str) -> (u8, usize, usize) {
    let mut it = s.split(' ');
    it.next();
    let n = it.next().unwrap().parse().unwrap();
    it.next();
    let from: usize = it.next().unwrap().parse().unwrap();
    it.next();
    let to: usize = it.next().unwrap().parse().unwrap();
    (n, from - 1, to - 1)
}
