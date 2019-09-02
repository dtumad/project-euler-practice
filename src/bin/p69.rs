#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use rayon::prelude::*;

fn solve(n: u64) -> u64 {
    let mut tester = PrimeTester::new();
    let mut result = 1;
    for i in 2.. {
        if tester.is_prime(i) {
            result *= i;
        }
        if result > n {
            return result / i;
        }
    }
    return 0;
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(10), 6);
    }
}
