#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Pow, Zero};

fn count_digits(a: u64, b: u64) -> u64 {
    let mut n = a.to_biguint().unwrap().pow(b);
    let mut r = 0;
    let ten = BigUint::parse_bytes(b"10", 10).unwrap();
    while !n.is_zero() {
        r += 1;
        n /= &ten;
    }
    return r;
}

// count the number of bases such that base^power has power digits
fn power_digit_count(power: u64) -> u64 {
    let mut base = 1;
    let mut length = 1;
    while length < power {
        base += 1;
        length = count_digits(base, power);
    }
    let mut r = 0;
    while length == power {
        r += 1;
        base += 1;
        length = count_digits(base, power);
    }
    return r;
}

fn solve(n: u64) -> u64 {
    (1..=n).map(|m| power_digit_count(m)).sum()
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_digits_test() {
        use super::count_digits;
        assert_eq!(count_digits(1, 150), 1);
        assert_eq!(count_digits(10, 3), 4);
        assert_eq!(count_digits(2, 4), 2);
    }
    #[test]
    fn power_digit_count_test() {
        use super::power_digit_count;
        assert_eq!(power_digit_count(1), 9);
        assert_eq!(power_digit_count(2), 6);
    }
}
