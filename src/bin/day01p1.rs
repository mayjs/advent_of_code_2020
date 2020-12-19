//! For this task, we have to find two numbers that add to 2020 in the input and return their product.

use advent_of_code_2020::get_input_vec;

const TARGET: u64 = 2020;

/// This employs a linear search for the two indices.
/// Given a sorted input, this will find the indices in O(n).
fn search(sorted: &Vec<u64>) -> (usize, usize) {
    let mut lower = 0;
    let mut upper = sorted.len() - 1;
    
    loop {
        assert!(lower < upper);
        let sum = sorted[lower] + sorted[upper];
        match sum.cmp(&TARGET) {
            std::cmp::Ordering::Less => lower += 1,
            std::cmp::Ordering::Equal => return (lower, upper),
            std::cmp::Ordering::Greater => upper -= 1,
        }
    }
}

fn main() {
    let mut input = get_input_vec::<u64>();
    input.sort();
    
    let (lower, upper) = search(&input);
    let lv = input[lower];
    let hv = input[upper];
    println!("Indices {} and {}: {} + {} = {}, {} * {} = {}", lower, upper, lv, hv, lv+hv, lv, hv, lv*hv);
}