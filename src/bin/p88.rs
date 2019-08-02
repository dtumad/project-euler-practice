#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One};

// for 1234.. calculates 1 + 2 + 3 + 4 + ..
fn sum_of_num(n: &BigUint) -> u64 {
    n.to_radix_le(10)
        .iter()
        .fold(0_u64, |acc, &d| acc + (d as u64))
}

fn product_of_num(n: &BigUint) -> u64 {
    n.to_radix_le(10)
        .iter()
        .fold(1_u64, |acc, &d| acc * (d as u64))
}

fn is_product_sum(n: &BigUint) -> Option<u64> {
    let p = product_of_num(n);
    if p == sum_of_num(n) {
        return Some(p);
    }
    return None;
}

// we know that the result is between k and 2k (11....11 -> 11...12k)
fn get_min_product_sum(mut k: usize) -> u64 {
    //let max: BigUint = pow(10.to_biguint().unwrap(), k);
    let mut min: BigUint = One::one();
    while k > 1 {
        let one: BigUint = One::one();
        min *= 10.to_biguint().unwrap();
        min += one;
        k -= 1;
    }
    let mut result = 900;
    for _ in 1..10000 {
        if let Some(new_result) = is_product_sum(&min) {
            result = std::cmp::min(result, new_result);
        }
        let one: BigUint = One::one();
        min += one;
    }
    return result
}

fn add_implied(mut k: usize, mut n: u64, product_sums: &mut Vec<Option<u64>>) -> () {
    while k < product_sums.len() {
        product_sums[k] = Some(n);
        k = k + n as usize - 1;
        n = n * 2;
    }
}
    
// IDEA: given that k -> n, we also have (k + n - 1) -> 2 * n
// This could speed up a lot of the calculations
// current solution still to slow, only gets to k = 1200 ish
use std::collections::HashSet;
fn solve(k_max: usize) -> u64 {
    let mut product_sums = vec![None; k_max + 1];
    for k in 2..=k_max {
        if let None = product_sums[k] {
            let n = get_min_product_sum(k);
            add_implied(k, n, &mut product_sums);
        }
    }
    return product_sums.iter()
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .collect::<HashSet<u64>>()
        .iter()
        .sum();
}

fn main() -> () {
    let result: u64 = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn digit_arithmetic_test() {
        use super::sum_of_num as sn;
        use super::product_of_num as pn;
        use super::ToBigUint;
        assert_eq!(sn(&11222.to_biguint().unwrap()), 8);
        assert_eq!(pn(&11222.to_biguint().unwrap()), 8);
        assert_eq!(sn(&1234.to_biguint().unwrap()), 10);
        assert_eq!(pn(&1234.to_biguint().unwrap()), 24);
    }
    #[test]
    fn get_min_product_sum_test() {
        use super::get_min_product_sum as mps;
        assert_eq!(mps(2), 4);
        assert_eq!(mps(3), 6);
        assert_eq!(mps(4), 8);
        assert_eq!(mps(5), 8);
        assert_eq!(mps(6), 12);
    }
}
