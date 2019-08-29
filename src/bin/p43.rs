#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_full_pangigital(mut n: u64) -> bool {
    if n < 1023456789 || n > 9876543210 {
        return false;
    }
    let mut digits: [bool; 10] = [false; 10];
    while n > 0 {
        let i = (n % 10) as usize;
        if digits[i] {
            return false;
        }
        digits[i] = true;
        n /= 10;
    }
    return true;
}

fn has_property(mut n: u64) -> bool {
    for div in vec![17, 13, 11, 7, 5, 3, 2].iter() {
        let smoll = n % 1000;
        if smoll % div != 0 {
            return false;
        }
        n /= 10;
    }
    return true;
}

fn solve() -> u64 {
    (1023456789_u64..=9876543210)
        .into_par_iter()
        .filter(|&n| (n / 1000000) % 2 == 0)
        .filter(|&n| is_full_pangigital(n) && has_property(n))
        .map(|n| {
            println!("{}", n);
            n
        })
        .sum()
}

fn main() -> () {
    let result = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_full_pangigital_test() {
        use super::is_full_pangigital as ifp;
        assert!(ifp(1023456789));
        assert!(ifp(1432705689));
        assert!(!ifp(201345678));
        assert!(!ifp(133456789));
    }
    #[test]
    fn has_property_test() {
        use super::has_property;
        assert!(has_property(1406357289));
        assert!(!has_property(1406375289));
    }
}
