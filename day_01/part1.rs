use std::io;
use std::cmp;
use std::collections::BinaryHeap;

const TOP_N: usize = 3;

fn main() {
    let mut heap = BinaryHeap::new();
    let mut curr: u32 = 0;
    for line in io::stdin().lines() {
        let l = line.unwrap();
        let line_trimed = l.trim();
        if line_trimed.is_empty() {
            push(&mut heap, curr);
            curr = 0;
            continue;
        }
        curr += line_trimed.parse::<u32>().unwrap();
    }
    push(&mut heap, curr);
    println!("total of top {}: {}", TOP_N, heap.iter().map(|cmp::Reverse(v)| {
        println!("{}", v);
        v
    }).sum::<u32>());
}

fn push(heap: &mut BinaryHeap<cmp::Reverse<u32>>, v: u32) {
   heap.push(cmp::Reverse(v));
   while heap.len() > TOP_N {
       heap.pop();
   }
}
