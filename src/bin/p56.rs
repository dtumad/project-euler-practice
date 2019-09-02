#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, Pow, ToPrimitive};

fn digit_sum(a: u64, b: u64) -> u64 {
    let mut n = a.to_biguint().unwrap().pow(b);
    let mut result = 0;
    let ten = BigUint::parse_bytes(b"10", 10).unwrap();
    while !n.is_zero() {
        result += (&n % &ten).to_u64().unwrap();
        n /= &ten;
    }
    return result;
}

fn solve(n: u64) -> u64 {
    let mut max_sum = 0;
    for a in (n/2)..n {
        for b in (n/2)..n {
            max_sum = std::cmp::max(max_sum, digit_sum(a, b));
        }
    }
    return max_sum;
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn digit_sum_test() {
        use super::digit_sum;
        assert_eq!(digit_sum(10,10), 1);
        assert_eq!(digit_sum(2,4), 7);
    }
}
