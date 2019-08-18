#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use std::collections::HashSet;

// returns true iff n contains all digits 1-9 once
fn is_pandigital(mut n: u64) -> bool {
    if n < 123456789 || n > 987654321 {
        return false;
    }
    let mut digits = 0;
    while n > 0 {
        let digit = n % 10;
        if digit == 0 {
            return false;
        }
        digits |= 1 << (digit - 1);
        n /= 10;
    }
    return digits == 0x1ff;
}

fn split_num(mut n: u64, mut break1: u64, mut break2: u64) -> [u64; 3] {
    let mut digit = 1;
    let mut result = [0, 0, 0];
    while break1 > 0 {
        result[0] += digit * (n % 10);
        digit *= 10;
        n /= 10;
        break1 -= 1;
        break2 -= 1;
    }
    digit = 1;
    while break2 > 0 {
        result[1] += digit * (n % 10);
        digit *= 10;
        n /= 10;
        break2 -= 1;
    }
    digit = 1;
    while n > 0 {
        result[2] += digit * (n % 10);
        digit *= 10;
        n /= 10;
    }
    return result;
}

fn get_splits(n: u64, set: &mut HashSet<u64>) -> () {
    for break1 in 1..=4 {
        for break2 in (break1 + 1)..=7 {
            let split = split_num(n, break1, break2);
            if split[0] * split[1] == split[2] {
                set.insert(split[2]);
            }
        }
    }
}

fn solve() -> u64 {
    let mut set: HashSet<u64> = HashSet::new();
    for i in (123456789..=987654321).filter(|&n| is_pandigital(n)) {
        get_splits(i, &mut set);
    }
    return set.iter().sum();
}

// this approach splits and then checks rather than combine then check
// would likely be faster the other way
fn main() -> () {
    let result = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn pandigital_test() {
        use super::is_pandigital;
        assert!(is_pandigital(123456789));
        assert!(is_pandigital(312645987));
        assert!(!is_pandigital(1234567890));
        assert!(!is_pandigital(12345678));
        assert!(!is_pandigital(123455789));
    }
    #[test]
    fn split_num_test() {
        use super::split_num;
        assert_eq!(split_num(123456789, 2, 4), [89, 67, 12345]);
        assert_eq!(split_num(123456789, 1, 8), [9, 2345678, 1]);
    }
}
