use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn get_digits(mut p: u64) -> [u8; 10] {
    let mut digits = [0; 10];
    while p > 0 {
        digits[(p % 10) as usize] += 1;
        p /= 10;
    }
    return digits;
}

fn is_prime_permute(p: u64, step: u64, pt: &mut PrimeTester) -> bool {
    let q = p + step;
    let r = q + step;
    (pt.is_prime(p) && pt.is_prime(q) && pt.is_prime(r))
        && get_digits(p) == get_digits(q)
        && get_digits(q) == get_digits(r)
}

fn solve(mut n: u64, init_max: u64, step_max: u64) -> (u64, u64, u64) {
    let mut pt = PrimeTester::new();
    for init in (1001..init_max).step_by(2) {
        for step in (2..step_max).step_by(2) {
            if is_prime_permute(init, step, &mut pt) {
                n -= 1;
                if n == 0 {
                    return (init, init + step, init + step + step);
                }
            }
        }
    }
    return (0, 0, 0);
}

fn main() -> () {
    let result = solve(get_arg(1), get_arg_else(2, 10000), get_arg_else(3, 4500));
    println!("{},{},{}", result.0, result.1, result.2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_prime_permute_test() {
        use super::is_prime_permute;
        use super::PrimeTester;
        let mut pt = PrimeTester::new();
        for i in 1..1000 {
            for j in 30..50 {
                assert!(!is_prime_permute(i, j, &mut pt));
            }
        }
        assert!(is_prime_permute(1487, 3330, &mut pt));
    }
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(1, 10000, 4500), (1487, 4817, 8147));
    }
}
