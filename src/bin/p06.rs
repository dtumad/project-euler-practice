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

// sum of 1..max
fn sum_nats(max: i64) -> i64 {
    return max * (max + 1) / 2;
}

// sum of 1^2..max^2
fn sum_square_nats(max: i64) -> i64 {
    return max * (max + 1) * (2 * max + 1) / 6;
}

fn sum_diff(max: i64) -> i64 {
    return sum_nats(max).pow(2) - sum_square_nats(max);
}

// Finds the difference between the sum of squares and square of sum
fn main() -> () {
    println!("{}", sum_diff(get_arg(1)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_nats_test() {
        use super::sum_nats;
        use super::sum_square_nats;
        assert_eq!(sum_nats(10), 55);
        assert_eq!(sum_square_nats(10), 385);
    }
    #[test]
    fn sum_diff_test() {
        use super::sum_diff;
        assert_eq!(sum_diff(10), 2640);
    }
}
