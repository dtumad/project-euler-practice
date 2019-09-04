#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_bouncy(mut n: u64) -> bool {
    let mut increase = false;
    let mut decrease = false;
    let mut last_digit = n % 10;
    n /= 10;
    while n > 0 {
        let d = n % 10;
        if d < last_digit {
            increase = true;
        } else if d > last_digit {
            decrease = true;
        }
        if increase && decrease {
            return true;
        }
        n /= 10;
        last_digit = d;
    }
    return false;
}

fn solve(bound: f64) -> u64 {
    let mut n = 0.;
    let mut bouncy = 0.;
    loop {
        n += 1.;
        if is_bouncy(n as u64) {
            bouncy += 1.;
        }
        // doesn't check strict equality
        if bouncy / n > bound {
            return (n - 1.) as u64;
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
    fn is_bouncy_test() {
        use super::is_bouncy as f;
        for i in 0..100 {
            assert!(!f(i));
        }
        assert!(f(12321));
        assert!(f(155349));
        assert!(!f(134468));
        assert!(!f(66420));
        assert!((1..1000).filter(|&n| f(n)).count() == 525);
    }
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(0.5), 538);
        assert_eq!(solve(0.9), 21780);
    }
}
