#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, ToPrimitive, Zero};

fn count_digits(mut n: BigUint) -> u64 {
    let mut r = 0;
    let ten = 10.to_biguint().unwrap();
    while !n.is_zero() {
        r += 1;
        n /= &ten;
    }
    return r;
}

fn get_digits(mut n: BigUint) -> [u8; 10] {
    let mut result = [0; 10];
    let ten = 10.to_biguint().unwrap();
    while !n.is_zero() {
        result[(&n % &ten).to_usize().unwrap()] += 1;
        n /= &ten;
    }
    return result;
}

use std::collections::HashMap;
fn solve(n: u8) -> String {
    let one: BigUint = One::one();
    let mut m: BigUint = One::one();
    let mut cube: BigUint = One::one();
    let mut length = 1;
    loop {
        let mut permutation_count: HashMap<[u8; 10], (u8, String)> = HashMap::new();
        while count_digits(cube.clone()) <= length {
            let digits = get_digits(cube.clone());
            permutation_count.insert(
                digits,
                match permutation_count.get(&digits) {
                    Some((i, old_cube)) => (i + 1, old_cube.clone()),
                    None => (1, cube.to_string()),
                },
            );
            m += &one;
            cube = &m * &m * &m;
        }
        match permutation_count
            .iter()
            .fold(None, |acc, (_, (count, small_cube))| {
                if *count == n {
                    Some(small_cube.clone())
                } else {
                    acc
                }
            }) {
            Some(solution) => return solution,
            None => length += 1,
        }
    }
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_digits_test() {
        use super::{get_digits, ToBigUint};
        assert_eq!(get_digits(123.to_biguint().unwrap())[1], 1);
        assert_eq!(get_digits(1223223221.to_biguint().unwrap())[2], 6);
        assert_eq!(get_digits(133433.to_biguint().unwrap())[3], 4);
    }
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(3), "41063625");
    }
}
