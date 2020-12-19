use std::io::{self, BufRead};

use regex::Regex;

// This solution is ugly, the whole fields vector thing is a really stupid way to do this. 
// Would have been better to create a quick struct with an appropriate setter function


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    
    let mut valid = 0;
    let mut fields: Vec<String> = Vec::new();
    let field_re = Regex::new(r"(\w+):[^\s]+").unwrap();
    let required = [
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"
    ];
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            if required.iter().all(|field| fields.iter().any(|pf| pf == field)) {
                valid += 1;
            }
            fields.clear();
        } else {
            field_re.captures_iter(&line)
                .for_each(|captures| fields.push(captures[1].to_string()));
        }
    }
    if required.iter().all(|field| fields.iter().any(|pf| pf == field)) {
        valid += 1;
    }
    
    println!("Valid passports: {}", valid);
}