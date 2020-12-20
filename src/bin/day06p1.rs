use std::{collections::HashSet, io::{self, BufRead}};


fn main() {
    let stdin = io::stdin();
    
    let mut current_group = HashSet::<char>::new();
    let mut group_counts = 0 as usize;
    for line in stdin.lock().lines().filter_map(Result::ok) {
        if line.len() == 0 {
            group_counts += current_group.len();
            current_group.clear();
        } else {
            line.chars().for_each(|q| {current_group.insert(q);});
        }
    }
    group_counts += current_group.len();
    
    println!("In total, the questions answered add up to {}", group_counts);
}