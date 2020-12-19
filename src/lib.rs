use std::{io::{self, BufRead}, str::FromStr};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_input_vec<T: FromStr>() -> Vec<T> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<T>())
        .filter_map(Result::ok)
        .collect()
}