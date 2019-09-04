#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_choose_bigger(n: u64, r: u64, bound: f64) -> bool {
    let mut a = n as f64;
    let mut b = r as f64;
    let mut c = (n - r) as f64;
    let mut product: f64 = 1.;
    while a > 0. {
        product *= a;
        a -= 1.;
        if b > 0. {
            product /= b;
            b -= 1.;
        }
        if c > 0. {
            product /= c;
            c -= 1.;
        }
        if product > bound {
            return true;
        }
    }
    return false;
}

fn solve(n: u64, bound: f64) -> u64 {
    (1..=n)
        .into_par_iter()
        .map(|nn| {
            (1..=nn)
                .filter(|&r| is_choose_bigger(nn, r, bound))
                .count() as u64
        })
        .sum()
}

fn main() -> () {
    let result = solve(get_arg_else(1, 100), get_arg_else(2, 1_000_000.));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_choose_bigger_test() {
        use super::is_choose_bigger;
        assert!(is_choose_bigger(23, 10, 1_000_000.));
        assert!(!is_choose_bigger(22, 9, 1_000_000.));
        assert!(is_choose_bigger(24, 10, 1_000_000.));
        assert!(!is_choose_bigger(23, 16, 1_000_000.));
    }
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(4, 3.), 3);
        assert_eq!(solve(4, 4.), 1);
        assert_eq!(solve(5, 4.), 5);
    }
}
