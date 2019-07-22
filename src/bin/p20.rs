#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};

// returns n! as a BigUint
fn big_factorial(n: u64) -> BigUint {
    (1..=n)
        .map(|m| m.to_biguint().unwrap())
        .product()
}

fn sum_digits(n: BigUint) -> u32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

fn main() -> () {
    let result: u32 = sum_digits(big_factorial(get_arg_else(1, 100)));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn big_factorial_test() {
        use super::big_factorial as bf;
        assert_eq!(bf(8).to_string(), "40320");
    }
    #[test]
    fn sum_digits_test() {
        use super::big_factorial as bf;
        use super::sum_digits as sd;
        assert_eq!(sd(bf(8)), 9);
    }
}
