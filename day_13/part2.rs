use std::iter::Peekable;
use std::str::Chars;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum List {
    List(Vec<List>),
    Num(u32),
}

macro_rules! list {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            List::List(v)
        }
    };
}

macro_rules! num {
    ($x:expr) => {
        List::Num($x)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Num(n1), List::Num(n2)) => n1.cmp(n2),
            (List::Num(n1), _) => list![num![*n1]].cmp(other),
            (_, List::Num(n2)) => self.cmp(&list![num![*n2]]),
            (List::List(v1), List::List(v2)) => {
                for (x, y) in v1.iter().zip(v2.iter()) {
                    let ord = x.cmp(y);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                v1.len().cmp(&v2.len())
            },
        }
    }
}

fn main() {
    let mut v = vec![];
    let mut it = std::io::stdin().lines();
    loop {
        let s1 = it.next().unwrap().unwrap();
        let s2 = it.next().unwrap().unwrap();
        v.push(parse(&s1));
        v.push(parse(&s2));
        if let None = it.next() {
            break; // line break
        }
    }
    let p1 = list![list![num![2]]];
    let p2 = list![list![num![6]]];
    v.push(p1.clone());
    v.push(p2.clone());
    v.sort();
    let i = v.iter().position(|x| *x == p1).unwrap() + 1;
    let j = v.iter().position(|x| *x == p2).unwrap() + 1;
    println!("{}", i * j);
}

fn parse(s: &str) -> List {
    let mut it = s.chars().peekable();
    parse_list(&mut it)
}

fn parse_list(it: &mut Peekable<Chars>) -> List {
    assert!(Some('[') == it.next());
    let mut v = vec![];
    while let Some(c) = it.peek() {
        match c {
            '[' => {
                v.push(parse_list(it));
            },
            ',' => {
                it.next();
            }
            ']' => {
                it.next();
                break;
            }
            _ if c.is_numeric() => {
                v.push(parse_num(it));
            },
            _ => panic!("unknown input"),
        }
    }
    List::List(v)
}

fn parse_num(it: &mut Peekable<Chars>) -> List {
    let s: String = it.take_while(|x| x.is_numeric()).collect();
    num![s.parse().unwrap()]
}
