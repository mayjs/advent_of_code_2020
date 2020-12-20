use std::{collections::HashMap, io::{self, BufRead}};


fn main() {
    let stdin = io::stdin();
    
    let mut current_group = HashMap::<char, usize>::new();
    let mut current_group_size = 0 as usize;
    let mut group_counts = 0 as usize;
    for line in stdin.lock().lines().filter_map(Result::ok) {
        if line.len() == 0 {
            group_counts += current_group.values().filter(|v| **v == current_group_size).count();
            current_group.clear();
            current_group_size = 0;
        } else {
            line.chars().for_each(|q| {
                current_group.insert(q, current_group.get(&q).unwrap_or(&0) + 1);
            });
            current_group_size += 1;
        }
    }
    group_counts += current_group.values().filter(|v| **v == current_group_size).count();
    
    println!("In total, the questions answered by all participants of their groups add up to {}", group_counts);
}