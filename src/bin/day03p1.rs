use std::io::{self, BufRead};

const TREE: char = '#';
const XSTEP: usize = 3;

fn main() {
    let stdin = io::stdin();
    let encounters = stdin.lock().lines().filter_map(Result::ok)
    .enumerate().filter(|(idx, line)| {
        let actual_pos = (idx * XSTEP) % line.len();
        line.as_bytes()[actual_pos] as char == TREE
    }).count();
    
    println!("We will encounter {} trees", encounters);
}