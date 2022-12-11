use std::str::FromStr;
use std::fmt::Debug;
use std::collections::VecDeque;

enum Op {
    Mul,
    Add,
    MulX(i32),
    AddX(i32),
}

enum Pred {
    Pred(i32, usize, usize)
}

fn main() {
    let (mut items, ops, tests) = parse();
    let mut cnts = vec![0; items.len()];
    for _ in 0..20 {
        for i in 0..items.len() {
            let mut item_list = std::mem::take(&mut items[i]); // move out of ownership
            while !item_list.is_empty() {
                let m = item_list.pop_front().unwrap();
                let n = eval_op(&ops[i], m);
                let j = eval_pred(&tests[i], n);
                let l = &mut items[j];
                l.push_back(n);
                cnts[i] += 1;
            }
            items[i] = item_list; // put back
        }
        println!("{:?}", items);
    }
    println!("{}", worry_level(&cnts));
}

fn worry_level(cnts: &Vec<u32>) -> u32 {
    let mut m1 = 0;
    let mut m2 = 0;
    for c in cnts {
        if *c > m1 {
            m2 = m1;
            m1 = *c;
        } else if *c > m2 {
            m2 = *c;
        }
    }
    return m1 * m2;
}

fn eval_op(op: &Op, x: i32) -> i32 {
    let ret = match op {
        Op::Mul => x * x,
        Op::Add => x + x,
        Op::MulX(y) => x * y,
        Op::AddX(y) => x + y,
    };
    ret / 3
}

fn eval_pred(pred: &Pred, x: i32) -> usize {
    let Pred::Pred(n, t, f) = *pred;
    if x % n == 0 {t} else {f}
}

fn parse() -> (
    Vec<VecDeque<i32>>,
    Vec<Op>,
    Vec<Pred>
    ) {
    let mut items: Vec<VecDeque<i32>> = vec![];
    let mut ops  : Vec<Op> = vec![];
    let mut tests: Vec<Pred> = vec![];

    let mut it = std::io::stdin().lines();
    loop {
        // Monkey
        it.next();

        // Starting items
        let s = it.next().unwrap().unwrap();
        items.push(parse_list(&s));

        // Operation
        let op = it.next().unwrap().unwrap();
        ops.push(parse_op(&op));

        // Test
        let test = it.next().unwrap().unwrap();
        let n: i32 = parse_last_num(&test);
        // If true
        let true_cond = it.next().unwrap().unwrap();
        let t_val: usize = parse_last_num(&true_cond);
        // If false
        let false_cond = it.next().unwrap().unwrap();
        let f_val: usize = parse_last_num(&false_cond);
        tests.push(Pred::Pred(n, t_val, f_val));

        if let None = it.next() {
            break; // no breakline
        }
    }
    (items, ops, tests)
}

fn parse_last_num<T: std::str::FromStr>(s: &str) -> T where <T as FromStr>::Err: Debug {
    s.trim().split(' ').last().unwrap().parse().unwrap()
}

fn parse_op(s: &str) -> Op {
    let v: Vec<&str> = s.trim().split(' ').collect();
    match v[v.len()-2..] {
        ["*", "old"] => Op::Mul,
        ["+", "old"] => Op::Add,
        ["*", n] => Op::MulX(n.parse().unwrap()),
        ["+", n] => Op::AddX(n.parse().unwrap()),
        _ => panic!("unknown pattern"),
    }
}

fn parse_list(s: &str) -> VecDeque<i32> {
    s.trim().strip_prefix("Starting items: ")
        .unwrap()
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect()
}
