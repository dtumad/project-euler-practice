// This doesn't correspond to a problem, essentially just a template file

use std::env;
use std::str::FromStr;

#[allow(dead_code)]
fn get_arg<T: FromStr>(arg_num: usize) -> T {
    let args: Vec<String> = env::args().collect();
    if args.len() <= arg_num {
        panic!("Not enough arguments, expected at least {}", arg_num);
    }
    return match (&args[arg_num]).parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not parse argument: {}", &args[arg_num]),
    };
}

// gets all primes less strictly less than the input
fn primes_below(n: usize) -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; n];
    primes[0] = false;
    primes[1] = false;
    for p in 2..n {
        if primes[p] {
            let mut q = p * p; // everything below square is already hit
            while q < n {
                primes[q] = false;
                q += p;
            }
        }
    }
    return primes;
}

fn sum_primes(primes: Vec<bool>) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    for &is_prime in primes.iter() {
        if is_prime {
            sum += i;
        }
        i = i + 1;
    }
    return sum;
}

// Sums all the primes below the given input
fn main() -> () {
    let result = sum_primes(primes_below(get_arg(1)));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn primes_below_test() {
        use super::primes_below;
        assert_eq!(
            primes_below(10),
            vec![false, false, true, true, false, true, false, true, false, false]
        );
        assert_eq!(primes_below(2), vec![false, false]);
    }
    #[test]
    fn sum_primes_test() {
        use super::*;
        assert_eq!(sum_primes(primes_below(10)), 17);
    }
}
