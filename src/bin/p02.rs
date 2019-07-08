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

// Finds the sum of even fibonacci numbers less than the input value
fn sum_fibs(max: i64) -> i64 {
    let mut current = (1,1);
    let result: i64 = std::iter::from_fn(move || {
        if current.1 <= max {
            current = (current.1, current.0 + current.1);
            Some(current)
        }
        else {
            None
        }
    }).map(|x| x.0)
    .filter(|x| x % 2 == 0)
    .sum();
    return result;
}

fn main() -> () {
    let max = get_arg::<i64>(1);
    let result = sum_fibs(max);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn fib_sum_test() {
        use super::sum_fibs;
        assert_eq!(44, sum_fibs(100));
        assert_eq!(10, sum_fibs(30));
        assert_eq!(44, sum_fibs(35));
    }
}
