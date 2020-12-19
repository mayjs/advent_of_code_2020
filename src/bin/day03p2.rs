use std::io::{self, BufRead};

const TREE: char = '#';

fn check_with_slope(lines: &Vec<String>, xstep: usize, ystep: usize) -> usize {
    lines.iter().enumerate().filter_map(|(idx, line)|
        match idx % ystep {
            0 => Some(line),
            _ => None,
        }  
    ).enumerate().filter(|(idx, line)| {
        let actual_pos = (idx * xstep) % line.len();
        line.as_bytes()[actual_pos] as char == TREE
    }).count()

}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<_>>();
    
    let encounter_prod: usize = [(1,1),
     (3,1),
     (5,1),
     (7,1),
     (1,2)
    ].iter().map(|(x, y)| 
        check_with_slope(&lines, *x as usize, *y as usize)
    ).product();
    
    println!("The product of all encounters is {}", encounter_prod);
}