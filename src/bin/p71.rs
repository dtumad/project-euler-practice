#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn reduce_numerator(mut n: u64, mut d: u64) -> u64 {
    for i in 2..d {
        while n % i == 0 && d % i == 0 {
            n /= i;
            d /= i;
        }
    }
    return n;
}

fn solve(target: f64, d_bound: u64) -> u64 {
    let (mut current_n, mut current_d, mut current_diff) = (0, 0, 1.);
    for d in 1..=d_bound {
        for n in 1..d {
            let f = n as f64 / d as f64;
            let this_diff = target - f;
            if this_diff < current_diff && this_diff > 0. {
                current_n = n;
                current_d = d;
                current_diff = this_diff;
            }
        }
    }
    reduce_numerator(current_n, current_d)
}

fn main() -> () {
    let result = solve(get_arg_else(1, 3./7.), get_arg_else(2, 1_000_000));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn reduce_numerator_test() {
        use super::reduce_numerator;
        assert_eq!(reduce_numerator(2, 12), 1);
        assert_eq!(reduce_numerator(6, 12), 1);
        assert_eq!(reduce_numerator(18, 24), 3);
        assert_eq!(reduce_numerator(7, 8), 7);
    }
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(3./7., 8), 2);
    }
}
