#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use rayon::prelude::*;

// n values failing this will never have the prime generating property
fn div_filter(n: u64) -> bool {
    if n % 2 == 1 { // If odd then n/1 + 1 is even and hence not prime
	return false;
    } else if n % 4 == 0 { // If divisible by 4 then n/2 + 2 is even and hence not prime
	return false;
    }
    let l = n % 10;
    return match l {
	2 | 8 => true,
	6 => n == 6, // Only 6 itself has this property and ends in 6
	0 => (n / 10) % 2 == 1, // Second to last digit must be odd since n/10 + 10 is prime
	_ => false
    };
}

fn is_prime_generating(n: u64, tester: &mut PrimeTester) -> bool {
    (1..=((n as f64).sqrt() as u64 + 5))
	.filter(|&d| n % d == 0)
	.all(|d| tester.is_prime(d + n / d))
}

fn solve(n: u64) -> u64 {
    let mut tester = PrimeTester::init(n as usize);
    return (1..=n)
	.filter(|&m| div_filter(m))
	.filter(|&m| is_prime_generating(m, &mut tester))
	.sum::<u64>();
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}
    
#[cfg(test)]
mod tests {
    use super::PrimeTester;
    #[test]
    fn div_filter_test() {
	use super::is_prime_generating as ipg;
	use super::div_filter;
	let mut tester = PrimeTester::new();
	for n in 7..10000 {
	    if ipg(n, &mut tester) {
		assert!(div_filter(n));
	    }
	}
    }
    #[test]
    fn prime_generating_test() {
	use super::is_prime_generating as ipg;
	let mut tester = PrimeTester::new();
	assert!(ipg(1, &mut tester));
	assert!(ipg(2, &mut tester));
	assert!(!ipg(3, &mut tester));
	assert!(ipg(30, &mut tester));
    }
}
