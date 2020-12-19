//! For this challenge, we receive lines in the form <MIN>-<MAX> <CHAR>: <PASSWORD>
//! And need to count passwords that contain <CHAR> between <MIN> and <MAX> times.
use std::io::{self, BufRead};

use regex::Regex;


fn main() {
    let stdin = io::stdin();
    
    let re = Regex::new(r"(^\d+)-(\d+) (\w): (\w+)").unwrap();
    let count = stdin.lock().lines().filter_map(Result::ok).filter_map(|line| {
        let captures = re.captures(&line).unwrap();
        let min = captures[1].parse::<usize>().unwrap();
        let max = captures[2].parse::<usize>().unwrap();
        let req_char = captures[3].chars().next().unwrap();
        
        let count = captures[4].chars().filter(|c| *c == req_char).count();
        if min <= count && count <= max {
            Some(())
        } else {
            None
        }
    }).count();
    println!("{} passwords meet their restrictions", count);
}