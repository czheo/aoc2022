use std::collections::HashMap;

enum Exp {
    Op(String, String, String),
    Num(i64),
}

fn main() {
    let prog = parse();
    println!("{}", eval("root", &prog));
}

fn eval(s: &str, prog: &HashMap<String, Exp>) -> i64 {
    match &prog[s] {
        Exp::Num(n) => *n,
        Exp::Op(op, a, b) => {
            match op.as_str() {
                "+" => eval(&a, prog) + eval(&b, prog),
                "-" => eval(&a, prog) - eval(&b, prog),
                "*" => eval(&a, prog) * eval(&b, prog),
                "/" => eval(&a, prog) / eval(&b, prog),
                _ => panic!()
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
                    Exp::Op(String::from(op), String::from(a), String::from(b)));
            },
            _ => panic!()
        }
    }
    ret
}
