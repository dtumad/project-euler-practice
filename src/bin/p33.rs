#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn div(n: u64, d: u64) -> f64 {
    (n as f64) / (d as f64)
}

fn is_simple_cancel(n: u64, d: u64) -> bool {
    let ratio = div(n, d);
    let n_one = n % 10;
    let n_ten = n / 10;
    let d_one = d % 10;
    let d_ten = d / 10;
    if n_one == d_ten && ratio == div(n_ten, d_one) {
        return true;
    }
    if n_ten == d_one && ratio == div(n_one, d_ten) {
        return true;
    }
    return false;
}

fn solve() -> u64 {
    let (mut a, mut b) = (10..100)
        .flat_map(|n| ((n + 1)..100).map(move |d| (n, d)))
        .filter(|&(n, d)| (n % 10 != 0) && (d % 10 != 0))
        .filter(|&(n, d)| is_simple_cancel(n, d))
        .fold((1, 1), |acc, (n, d)| (acc.0 * n, acc.1 * d));
    for i in 2..a {
        while a % i == 0 && b % i == 0 {
            a /= i;
            b /= i;
        }
    }
    return b;
}

fn main() -> () {
    let result: u64 = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn cancel_test() {
        use super::is_simple_cancel as f;
        assert!(f(49, 98));
        assert!(!f(30, 50));
    }
}
