//! For this challenge, we receive lines in the form <MIN>-<MAX> <CHAR>: <PASSWORD>
//! And need to count passwords that contain <CHAR> between <MIN> and <MAX> times.
use std::io::{self, BufRead};

use regex::Regex;

fn check_char(inp_bytes: &[u8], req_char: char, idx: usize) -> bool {
    inp_bytes.get(idx).map(|c| req_char == *c as char).unwrap_or(false)
}

fn main() {
    let stdin = io::stdin();
    
    let re = Regex::new(r"(^\d+)-(\d+) (\w): (\w+)").unwrap();
    let count = stdin.lock().lines().filter_map(Result::ok).filter_map(|line| {
        let captures = re.captures(&line).unwrap();
        let first_idx = captures[1].parse::<usize>().unwrap() - 1;
        let second_idx = captures[2].parse::<usize>().unwrap() - 1;
        let req_char = captures[3].chars().next().unwrap();
        
        let inp_bytes = captures[4].as_bytes();
        
        if check_char(inp_bytes, req_char, first_idx) ^ check_char(inp_bytes, req_char, second_idx) {
            Some(())
        } else {
            None
        }
    }).count();
    println!("{} passwords meet their restrictions", count);
}