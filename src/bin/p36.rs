#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use rayon::prelude::*;

fn get_digits(mut n: u64, base: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(10);
    while n > 0 {
        result.push(n % base);
        n /= base;
    }
    return result;
}

fn is_palindrome(n: u64, base: u64) -> bool {
    let digits = get_digits(n, base);
    let l = digits.len();
    return (0..(l / 2)).fold(true, |acc, i| acc && digits[i] == digits[l - i - 1]);
}

fn solve(n: u64) -> u64 {
    (1..=n)
        .into_par_iter()
        .filter(|&m| is_palindrome(m, 10) && is_palindrome(m, 2))
        .sum()
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_palindrome_test() {
        use super::is_palindrome;
        assert!(is_palindrome(1234321, 10));
        assert!(is_palindrome(123321, 10));
        assert!(is_palindrome(1, 10));
        assert!(!is_palindrome(123, 10));
        assert!(!is_palindrome(12345321, 10));

        assert!(is_palindrome(0b11011, 2));
        assert!(is_palindrome(0b10011001, 2));
        assert!(is_palindrome(0b1, 2));
        assert!(!is_palindrome(0b11001, 2));
        assert!(!is_palindrome(0b11010, 2));
    }
}
