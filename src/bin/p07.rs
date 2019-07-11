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

fn get_nth_prime(n: usize) -> u64 {
    let mut primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13];
    let mut next: u64 = 14;
    while primes.len() < n {
        let mut next_is_prime = true;
        for p in primes.iter() {
            if next % p == 0 {
                next_is_prime = false;
                break;
            }
        }

        if next_is_prime {
            primes.push(next);
        }

        next = next + 1;
    }
    return primes[n - 1];
}

// Finds the nth prime number
fn main() -> () {
    println!("{}", get_nth_prime(get_arg(1)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn nth_prime_test() {
        use super::get_nth_prime as f;
        assert_eq!(f(1), 2);
        assert_eq!(f(7), 17);
        assert_eq!(f(100), 541);
    }
}
