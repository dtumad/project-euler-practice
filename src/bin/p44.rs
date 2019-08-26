#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_pent(n: u64) -> bool {
    let m = (((24 * n + 1) as f64).sqrt() + 1.) / 6.;
    return (m as u64) as f64 == m;
}

fn exists_with_diff(diff: u64) -> bool {
    let mut n = 1;
    loop {
        let p1 = n * (3 * n - 1) / 2;
        let mut p2 = (n + 1) * (3 * n + 2) / 2;
        if p2 - p1 > diff {
            return false;
        } else if p2 - p1 == diff && is_pent(p2 + p1) {
            return true;
        } else {
            let mut m = 1;
            while p2 < p1 + diff {
                p2 = (n + m) * (3 * (n + m) - 1) / 2;
                if p2 - p1 == diff && is_pent(p2 + p1) {
                    return true;
                }
                m += 1;
            }
        }
        n += 1;
    }
}

fn solve() -> u64 {
    (1..)
        .map(|n| n * (3 * n - 1) / 2)
        .filter(|&n| exists_with_diff(n))
        .next()
        .unwrap()
}

fn main() -> () {
    let result = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_pent_test() {
        use super::is_pent;
        for n in 1..100 {
            assert!(is_pent(n * (3 * n - 1) / 2));
            assert!(!is_pent(n * (3 * n - 1) / 2 + 1));
        }
    }
    #[test]
    fn exists_with_diff_test() {
        use super::exists_with_diff as ewd;
        assert!(ewd(5482660));
        assert!(!ewd(5482659));
    }
}
