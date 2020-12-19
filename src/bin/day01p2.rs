//! The same as before, but this time with three numbers.

use advent_of_code_2020::get_input_vec;

const TARGET: u64 = 2020;

// TODO: Maybe there is a way to implement this in a better way, were we move three cursors on the vector.
//       How would we decide which cursor to move though?

/// Given a third index, find two indicies so that the three numbers in the vector will add up to 2020
fn search_two(sorted: &Vec<u64>, third_idx: usize) -> Option<(usize, usize, usize)> {
    let mut lower = 0;
    let mut upper = sorted.len() - 1;
    
    let target = TARGET - sorted[third_idx];
    
    loop {
        if lower >= upper {
            return None;
        }
        let sum = sorted[lower] + sorted[upper];
        match sum.cmp(&target) {
            std::cmp::Ordering::Less => lower += 1,
            std::cmp::Ordering::Equal => return Some((lower, upper, third_idx)),
            std::cmp::Ordering::Greater => upper -= 1,
        }
        if lower == third_idx {
            lower += 1;
        }
        if upper == third_idx {
            upper -= 1;
        }
    }
}

/// Brute force the third index, but use our more efficient algorithm from before
fn search(sorted: &Vec<u64>) -> (usize, usize, usize) {
    (1..sorted.len()).filter_map(|third| search_two(sorted, third)).next().unwrap()
}

fn main() {
    let mut input = get_input_vec::<u64>();
    input.sort();
    
    let (first, second, third) = search(&input);
    let fv = input[first];
    let sv = input[second];
    let tv = input[third];
    println!("Indices {}, {}, {}: {} + {} + {} = {}, Product = {}", first, second, third, fv, sv, tv, fv + sv + tv, fv * sv * tv);
}