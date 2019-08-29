#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn get_digits(mut n: u64) -> [u8; 10] {
    let mut result = [0; 10];
    while n > 0 {
        result[(n % 10) as usize] += 1;
        n /= 10;
    }
    return result;
}

// true if n,2n,...,mn all have the same digits
fn digits_match(n: u64, mut m: u64) -> bool {
    let digits = get_digits(n);
    while m > 1 {
        if digits != get_digits(m * n) {
            return false;
        }
        m -= 1;
    }
    return true;
}

// only solves for answers beginning with 1
fn solve(m: u64) -> u64 {
    let mut s = 10;
    loop {
        match (s..(2 * s)).filter(|&n| digits_match(n, m)).next() {
            Some(r) => return r,
            None => (),
        }
        s *= 10;
    }
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn digits_match_test() {
        use super::digits_match;
        assert!(digits_match(125874, 2));
        assert!(!digits_match(125874, 3));
        assert!(!digits_match(125875, 2));
    }
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 2);
    }
}
