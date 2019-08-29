#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;


fn reverse(mut n: BigUint) -> BigUint {
    let mut m = Zero::zero();
    let ten = BigUint::parse_bytes(b"10", 10).unwrap();
    while !n.is_zero() {
        m *= &ten;
        m += &n % &ten;
        n /= &ten;
    }
    return m;
}

fn is_lychrel(n: u64) -> bool {
    let mut big_n = n.to_biguint().unwrap();
    big_n += reverse(big_n.clone());
    for _ in 0..50 {
        let m = reverse(big_n.clone());
        if big_n == m {
            return false;
        }
        big_n += m;
    }
    return true;
}

fn solve(n: u64) -> u64 {
    (1..=n)
        .into_par_iter()
        .filter(|&m| is_lychrel(m))
        .count() as u64
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_test() {
        use super::reverse;
        use super::BigUint;
        assert_eq!(reverse(BigUint::parse_bytes(b"99899", 10).unwrap()).to_string(), "99899");
        assert_eq!(reverse(BigUint::parse_bytes(b"12345", 10).unwrap()).to_string(), "54321");
    }
    #[test]
    fn is_lychrel_test() {
        use super::is_lychrel;
        assert!(is_lychrel(4994));
        assert!(is_lychrel(196));
        assert!(!is_lychrel(47));
        assert!(!is_lychrel(349));
    }
}
