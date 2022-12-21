use std::collections::HashMap;

enum Exp {
    Op(String, String, String),
    Num(i64),
}

fn main() {
    let prog = parse();
    println!("{:?}", eval("root", &prog));
}

fn eval(s: &str, prog: &HashMap<String, Exp>) -> Option<i64> {
    if s == "humn" {
        return None;
    }
    match &prog[s] {
        Exp::Num(n) => Some(*n),
        Exp::Op(op, a, b) => {
            if let (Some(x), Some(y)) = (eval(&a, prog), eval(&b, prog)) {
                match op.as_str() {
                    "+" => Some(x + y),
                    "-" => Some(x - y),
                    "*" => Some(x * y),
                    "/" => Some(x / y),
                    _ => panic!(),
                }
            } else if op == "=" {
                assert!(s == "root");
                if let Some(n) = eval(&a, prog) {
                    Some(back_eval(&b, n, prog))
                } else if let Some(n) = eval(&b, prog) {
                    Some(back_eval(&a, n, prog))
                } else {
                    panic!()
                }
            } else {
                None
            }
        }
    }
}

fn back_eval(s: &str, n: i64, prog: &HashMap<String, Exp>) -> i64 {
    if s == "humn" {
        return n;
    }
    match &prog[s] {
        Exp::Num(n) => *n,
        Exp::Op(op, a, b) => {
            match op.as_str() {
                "+" => {
                    if let Some(x) = eval(&a, prog) {
                        back_eval(&b, n - x, prog)
                    } else if let Some(x) = eval(&b, prog) {
                        back_eval(&a, n - x, prog)
                    } else {
                        panic!()
                    }
                },
                "-" => {
                    if let Some(x) = eval(&a, prog) {
                        back_eval(&b, x - n, prog)
                    } else if let Some(x) = eval(&b, prog) {
                        back_eval(&a, n + x, prog)
                    } else {
                        panic!()
                    }
                },
                "*" => {
                    if let Some(x) = eval(&a, prog) {
                        back_eval(&b, n / x, prog)
                    } else if let Some(x) = eval(&b, prog) {
                        back_eval(&a, n / x, prog)
                    } else {
                        panic!()
                    }
                },
                "/" => {
                    if let Some(x) = eval(&a, prog) {
                        back_eval(&b, x / n, prog)
                    } else if let Some(x) = eval(&b, prog) {
                        back_eval(&a, x * n, prog)
                    } else {
                        panic!()
                    }
                },
                _ => panic!(),
            }
        }
    }
}

fn parse() -> HashMap<String, Exp>{
    let mut ret = HashMap::new();
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[..] {
            [name, n] => {
                ret.insert(
                    String::from(&name[..name.len()-1]),
                    Exp::Num(n.parse().unwrap()));
            },
            [name, a, op, b] => {
                ret.insert(
                    String::from(&name[..name.len()-1]),
                    Exp::Op(if name == "root:" {
                        String::from("=")
                    } else {
                        String::from(op)
                    }, String::from(a), String::from(b)));
            },
            _ => panic!()
        }
    }
    ret
}
