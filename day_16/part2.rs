use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let (flows, graph, start) = parse();
    let mut cache = HashMap::new();
    let ans = dfs(start, 1<<start, 26, false, &flows, &graph, &mut cache, start);
    println!("{}", ans);
    println!("stats searched = {}", cache.len());
}

fn dfs(node: usize, visited: u64, time: u64, elephant: bool,
    flows: &Vec<u64>, graph: &Vec<Vec<usize>>,
    cache: &mut HashMap<(usize, u64, u64, bool), u64>,
    start: usize) -> u64 {
    if time <= 0 {
        if elephant {
            return 0;
        } else {
            return dfs(start, visited, 26, true, flows, graph, cache, start);
        }
    }
    let key = (node, visited, time, elephant);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let mut ret = 0;
    if flows[node] > 0 && !is_visisted(node, visited) {
        let v = set_visisted(node, visited);
        ret = max(ret, flows[node] * (time - 1) + dfs(node, v, time - 1, elephant, flows, graph, cache, start));
    }
    for nb in graph[node].iter() {
        ret = max(ret, dfs(*nb, visited, time - 1, elephant, flows, graph, cache, start));
    }
    cache.insert(key, ret);
    ret
}

fn is_visisted(node: usize, visited: u64) -> bool {
    1 << node & visited == 1 << node
}

fn set_visisted(node: usize, visited: u64) -> u64 {
    1 << node | visited
}

fn parse()->(Vec<u64>, Vec<Vec<usize>>, usize) {
    let mut names = HashMap::new();
    let mut neighbors = vec![];
    let mut flows = vec![];
    let mut graph = vec![];
    let mut idx = 0;
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        if let [s1, s2] = line.split("; ").collect::<Vec<&str>>()[..] {
            let (node, rate) = parse_rate(s1);
            let nbs = parse_neighbors(s2);
            names.insert(String::from(node), idx);
            flows.push(rate);
            neighbors.push(nbs);
        } else {
            panic!("bad input: {}", line);
        }
        idx += 1;
    }
    for i in 0..idx {
        graph.push(neighbors[i].iter().map(|x| names[x]).collect());
    }
    (flows, graph, names["AA"])
}

fn parse_rate(s: &str) -> (&str, u64){
    let i = s.find(' ').unwrap();
    let node = &s[i+1..i+3];
    let i = s.find('=').unwrap();
    let rate = s[i+1..].parse().unwrap();
    (node, rate)
}

fn parse_neighbors(s: &str) -> Vec<String> {
    let mut i = 0;
    for _ in 0..4 {
        i += s[i..].find(' ').unwrap() + 1;
    }
    s[i..].split(", ").map(|x| String::from(x)).collect()
}
