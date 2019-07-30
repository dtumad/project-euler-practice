#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

extern crate num_bigint;
extern crate num_traits;
use num_bigint::ToBigUint;
use num_traits::Pow;

fn solve(a_max: u64, b_max: u64) -> u64 {
    let mut hs = std::collections::HashSet::new();
    for a in 2..=a_max {
        for b in 2..=b_max {
            hs.insert(a.to_biguint().unwrap().pow(b));
        }
    }
    return hs.len() as u64;
}

fn main() -> () {
    let a_max = get_arg_else(1, 5);
    let b_max = get_arg_else(2, 5);
    let result = solve(a_max, b_max);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(2,2), 1);
        assert_eq!(solve(3,3), 4);
        assert_eq!(solve(4,4), 8);
        assert_eq!(solve(5,5), 15);
        assert_eq!(solve(10,10), 69);
        assert_eq!(solve(50,50), 2184);
        assert_eq!(solve(75,75), 5098);
    }
}
