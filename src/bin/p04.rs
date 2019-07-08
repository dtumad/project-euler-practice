// This doesn't correspond to a problem, essentially just a template file

use std::env;
use std::str::FromStr;

#[allow(dead_code)]
fn get_arg <T: FromStr> (arg_num: usize) -> T {
    let args: Vec<String> = env::args().collect();
    if args.len() <= arg_num {
        panic!("Not enough arguments, expected at least {}", arg_num);
    }
    return match (&args[arg_num]).parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not parse argument: {}", &args[arg_num])
    }
}

// returns the nth digit from the 1s place, with 1s place being 0
fn get_nth_digit(number: u32, digit: u32) -> u32 {
    return (number / (10_u32.pow(digit))) % 10;
}

fn get_num_digits(mut number: u32) -> u32 {
    let mut num_digits = 0;
    while number > 0 {
        num_digits += 1;
        number /= 10;
    }
    return num_digits;
}

fn is_palindrome(number: u32) -> bool {
    let num_digits = get_num_digits(number);
    for i in 0..(num_digits/2) {
        if get_nth_digit(number, i) != get_nth_digit(number, num_digits-i-1) {
            return false;
        }
    }
    return true;
}

// returns true if the number is a product of two n digit numbers
fn is_product_of_two_ndigits(number: u32, n: u32) -> bool {
    // these are the low and high values of n digit numbers (e.g. 10-99)
    let lowest_ndigit = 10_u32.pow(n-1);
    let highest_ndigit = (lowest_ndigit * 10) - 1;

    // these are the lowest divisors that could be valid
    let mut lower_bound = number / highest_ndigit;
    let mut upper_bound = number / lowest_ndigit;
    
    // reset if the bounds have been moved too far
    if lower_bound < lowest_ndigit {
        lower_bound = lowest_ndigit;
    }
    if upper_bound > highest_ndigit {
        upper_bound = highest_ndigit;
    }

    for i in lower_bound..=upper_bound {
        if number % i == 0 {
            return true;
        }
    }
    return false;
}

// Finds the largest palindrome that is the product of two n digit numbers
fn main() -> () {
    let n: u32 = get_arg(1);
    for i in (1..(10_u32.pow(n) - 1).pow(2)).rev() {
        if is_palindrome(i) && is_product_of_two_ndigits(i, n) {
            println!("Largest palindrome found is {}", i);
            return;
        }
    }
    println!("Something's wrong, nothing found");
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_palindrome_test() {
        use super::is_palindrome;
        assert_eq!(true, is_palindrome(121));
        assert_eq!(true, is_palindrome(12344321));
        assert_eq!(false, is_palindrome(12345));
        assert_eq!(false, is_palindrome(12345321));
    }
    #[test]
    fn is_product_test() {
        use super::is_product_of_two_ndigits as f;
        assert_eq!(true, f(100, 2));
        //assert_eq!(false, f(101, 2));
        for i in 100..120 {
            for j in 900..1000 {
                assert_eq!(true, f(i*j, 3));
                assert_eq!(false, f(i*j, 2));
            }
        }
    }
}
