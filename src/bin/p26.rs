#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use rayon::prelude::*;

// lenght of the repeating portion of 1/d
fn get_cycle(d: u64) -> usize {
    let mut digits = Vec::new();
    let mut n = 1;
    while !digits.contains(&n) {
        digits.push(n);
        n = (n * 10) % d;
        if n == 0 {
            return 0;
        }
    }
    return digits.len() - digits.iter().position(|&m| m == n).unwrap();
}

fn solve(n: u64) -> u64 {
    (2..=n)
        .into_par_iter()
        .map(|d| (get_cycle(d), d))
        .max()
        .unwrap()
        .1
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn cycle_test() {
        use super::get_cycle;
        assert_eq!(get_cycle(2), 0);
        assert_eq!(get_cycle(3), 1);
        assert_eq!(get_cycle(6), 1);
        assert_eq!(get_cycle(7), 6);
    }
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(10), 7);
    }
}
