#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_concealed(mut n: u64) -> bool {
    if !(n % 10 == 0) {
        return false;
    }
    n /= 100;
    for i in 1..=9 {
        if !(n % 10 == 10 - i) {
            return false;
        }
        n /= 100;
    }
    return true;
}

fn solve() -> u64 {
    ((1010101010..)
        .step_by(10)
        .map(|n| n * n)
        .filter(|&n| is_concealed(n))
        .next()
        .unwrap()
        as f64).sqrt() as u64
}

fn main() -> () {
    let result = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_concealed_test() {
        use super::is_concealed;
        for i in 1020304050607080901..1020304050607080909 {
            assert!(!is_concealed(i));
        }
        for i in 102030..102090 {
            assert!(!is_concealed(i));
        }
        assert!(is_concealed(1020304050607080900));
        assert!(is_concealed(1223344556677889900));
    }
}
