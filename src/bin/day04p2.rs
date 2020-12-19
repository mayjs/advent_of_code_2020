use std::io::{self, BufRead};

use regex::Regex;

#[derive(Debug, Default)]
struct PassportFields {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>
}

impl PassportFields {
    fn put(&mut self, field: &str, value: String) {
        match field {
            "byr" => self.byr.replace(value),
            "iyr" => self.iyr.replace(value),
            "eyr" => self.eyr.replace(value),
            "hgt" => self.hgt.replace(value),
            "hcl" => self.hcl.replace(value),
            "ecl" => self.ecl.replace(value),
            "pid" => self.pid.replace(value),
            _ => None,
        };
    }
    
    fn check_range(val: &str, min: usize, max: usize) -> bool {
        val.parse::<usize>().map(|num| num >= min && num <= max).unwrap_or(false)
    }
    
    // This is pretty inefficient due to constant recompiling of the regular expressions.
    // We could also move them into a lazy_static, but to be honest this puzzle was kinda annoying anyway ¯\_(ツ)_/¯
    fn validate(&self) -> bool {
        let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let pid_re = Regex::new(r"^\d{9}$").unwrap();
        [
            self.byr.as_ref().map(|byr|
                byr.len() == 4 && Self::check_range(byr, 1920, 2002)
            ),
            self.iyr.as_ref().map(|iyr|
                iyr.len() == 4 && Self::check_range(iyr, 2010, 2020)
            ),
            self.eyr.as_ref().map(|eyr|
                eyr.len() == 4 && Self::check_range(eyr, 2020, 2030)
            ),
            self.hgt.as_ref().map(|hgt| {
                    let height_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();   
                    let maybe_captures = height_re.captures(hgt);
                    if let Some(captures) = maybe_captures {
                        match &captures[2] {
                            "cm" => Self::check_range(&captures[1], 150, 193),
                            "in" => Self::check_range(&captures[1], 59, 76),
                            _ => panic!(),
                        }
                    } else {
                        false
                    }
                }
            ),
            self.hcl.as_ref().map(|hcl| hcl_re.is_match(hcl)),
            self.ecl.as_ref().map(|ecl| valid_ecl.iter().any(|ve| ve == ecl)),
            self.pid.as_ref().map(|pid| pid_re.is_match(pid)),

        ].iter().all(|opt| opt.unwrap_or(false))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    
    let mut valid = 0;
    let mut fields = PassportFields::default();
    let field_re = Regex::new(r"(\w+):([^\s]+)").unwrap();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            if fields.validate() {
                valid += 1;
            }
            fields = PassportFields::default();
        } else {
            field_re.captures_iter(&line)
                .for_each(|captures| fields.put(&captures[1], captures[2].to_string()));
        }
    }
    if fields.validate() {
        valid += 1;
    }
    
    println!("Valid passports: {}", valid);
}