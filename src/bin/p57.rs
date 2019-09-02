#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero};

fn count_digits(mut n: BigUint) -> u64 {
    let mut r = 0;
    let ten = BigUint::parse_bytes(b"10", 10).unwrap();
    while !n.is_zero() {
        r += 1;
        n /= &ten;
    }
    return r;
}

fn solve(mut n: u64) -> u64 {
    let mut frac = (3.to_biguint().unwrap(), 2.to_biguint().unwrap());
    std::iter::from_fn(move || {
        n -= 1;
        if n == 0 {
            None
        } else {
            frac = (&frac.0 + &frac.1 + &frac.1, &frac.0 + &frac.1);
            Some(frac.clone())
        }
    })
    .filter(|(num, div)| count_digits(num.clone()) > count_digits(div.clone()))
    .count() as u64
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
        use super::ToBigUint;
        assert_eq!(count_digits(1.to_biguint().unwrap()), 1);
        assert_eq!(count_digits(9.to_biguint().unwrap()), 1);
        assert_eq!(count_digits(10.to_biguint().unwrap()), 2);
        assert_eq!(count_digits(99.to_biguint().unwrap()), 2);
        assert_eq!(count_digits(100.to_biguint().unwrap()), 3);
        assert_eq!(count_digits(999.to_biguint().unwrap()), 3);
    }
}
