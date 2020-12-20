use std::{io::{self, BufRead}, str::FromStr};

fn div_ceil(a: usize, b: usize) -> usize {
    let remainder = a % b;
    if remainder == 0 {
        a / b
    } else {
        (a / b) + 1
    }
}

fn follow_path(path: &str, symb_lower: char, symb_upper: char, range: usize) -> usize {
    let mut lower: usize = 0;
    let mut upper: usize = range;

    for step in path.chars() {
        if step == symb_lower {
            upper = lower + (upper-lower)/2;
        } else if step == symb_upper {
            lower = lower + div_ceil(upper-lower, 2);
        }
    }
    assert!(lower == upper);
    lower
}

#[derive(Debug)]
struct Seat {
    raw: String,
    column: usize,
    row: usize,
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            raw: s.to_string(),
            row: follow_path(&s[0..7], 'F', 'B', 127),
            column: follow_path(&s[7..], 'L', 'R', 7)
        })
    }
}

impl Seat {
    fn get_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn main() {
    let stdin = io::stdin();
    let max_seat = stdin.lock().lines()
        .filter_map(Result::ok)
        .map(|l| Seat::from_str(&l))
        .filter_map(Result::ok)
        .max_by_key(|seat| seat.get_id()).unwrap();
    println!("Highest occuring seat id is {:?} with id {}", max_seat, max_seat.get_id());
}