use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_truncatable_prime(mut p: u64, tester: &mut PrimeTester) -> bool {
    let mut q = p;
    // truncate right to left
    while p > 0 {
        if !tester.is_prime(p) {
            return false;
        }
        p /= 10;
    }
    let mut f = 10_u64.pow((q as f64).log10() as u32);
    while q > 9 {
        q %= f;
        if !tester.is_prime(q) {
            return false;
        }
        f /= 10;
    }
    return true;
}

fn solve(n: usize) -> u64 {
    let mut tester = PrimeTester::new();
    (23..)
        .filter(|&m| is_truncatable_prime(m, &mut tester))
        .take(n)
        .sum()
}

fn main() -> () {
    let result: u64 = solve(get_arg_else(1, 11));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_truncatable_prime_test() {
        use super::{is_truncatable_prime, PrimeTester};
        let mut tester = PrimeTester::new();
        assert!(is_truncatable_prime(3797, &mut tester));
        assert!(is_truncatable_prime(37, &mut tester));
        assert!(!is_truncatable_prime(3793, &mut tester));
    }
}
