#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

use project_euler_practice::prime::PrimeTester;
use std::collections::HashSet;

fn has_safe_digits(mut n: u64) -> bool {
    while n > 0 {
        match n % 10 {
            0 | 2 | 4 | 6 | 8 | 5 => return false,
            _ => (),
        }
        n /= 10;
    }
    return true;
}

fn len(n: u64) -> u32 {
    (n as f64).log10() as u32
}

fn cycle_number(n: u64, shift: u32) -> u64 {
    let l = len(n);
    let s = 10_u64.pow(shift);
    return (n / s) + ((n % s) * 10_u64.pow(l - shift + 1));
}

fn is_circular_prime(n: u64, tester: &HashSet<u64>) -> bool {
    if n < 11 {
        n == 2 || n == 3 || n == 5 || n == 7
    } else {
        has_safe_digits(n) &&
            (0..=len(n))
                .map(|shift| tester.contains(&cycle_number(n, shift)))
                .all(|x| x)
    }
}

fn solve(n: u64) -> usize {
    let pt = PrimeTester::init(n as usize).get_primes(n as usize);
    (2..=n)
        .into_par_iter()
        .filter(|&p| p % 2 != 0 && is_circular_prime(p, &pt))
        .count()
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_safe_digits_test() {
        use super::has_safe_digits as hsd;
        assert!(hsd(1379));
        assert!(!hsd(13579));
        assert!(!hsd(1039));
        assert!(!hsd(617));
    }
    #[test]
    fn cycle_number_test() {
        use super::cycle_number as cn;
        assert_eq!(cn(1234567, 1), 7123456);
        assert_eq!(cn(1234567, 2), 6712345);
        assert_eq!(cn(1234567, 3), 5671234);
    }
    #[test]
    fn is_circular_prime_test() {
        use super::is_circular_prime as icp;
        use super::PrimeTester;
        let mut pt = PrimeTester::init(100).get_primes(100);
        assert!(icp(79, &mut pt));
        assert!(icp(97, &mut pt));
        assert!(icp(11, &mut pt));
        assert!(icp(17, &mut pt));
        assert!(!icp(19, &mut pt));
        assert!(!icp(23, &mut pt));
    }
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(100), 13);
    }
}
