#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use rayon::prelude::*;

fn is_pangigital(mut n: u64) -> bool {
    let mut digits: [bool; 10] = [false; 10];
    let mut len = 0;
    while n > 0 {
        digits[(n % 10) as usize] = true;
        len += 1;
        n /= 10;
    }
    if digits[0] {
        return false
    }
    for i in 1..=len {
        if !digits[i] {
            return false;
        }
    }
    for i in (len+1)..=9 {
        if digits[i] {
            return false;
        }
    }
    return true;
}

// largest pandigital prime less than n
fn solve(mut n: u64) -> u64 {
    if n % 2 == 0 {
        n += 1;
    }
    let mut pt = PrimeTester::init(n as usize);
    while n > 0 {
        if is_pangigital(n) && pt.is_prime(n) {
            return n;
        }
        n -= 2;
    }
    return 0;
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_pangigital_test() {
        use super::is_pangigital;
        assert!(is_pangigital(1));
        assert!(is_pangigital(123));
        assert!(is_pangigital(2413));
        assert!(is_pangigital(623541));
        assert!(!is_pangigital(12034));
        assert!(!is_pangigital(1235));
        assert!(!is_pangigital(1623548));
        assert!(!is_pangigital(2));
    }
}
