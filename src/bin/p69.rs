#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::is_relatively_prime;
#[allow(unused_imports)]
use rayon::prelude::*;

fn totient(n: u64) -> u64 {
    (1..n)
        .filter(|&m| is_relatively_prime(m, n))
        .count() as u64
}

fn solve(n: u64) -> u64 {
    (2..=n)
        .step_by(2)
        .map(|m| (m, m/totient(m)))
        .fold((0, 0), |acc, elem| {
            if elem.1 > acc.1 {
                elem
            }
            else {
                acc
            }
        }).0
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn totient_test() {
        use super::totient;
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(4), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(6), 2);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(8), 4);
        assert_eq!(totient(9), 6);
        assert_eq!(totient(10), 4)
    }
}
