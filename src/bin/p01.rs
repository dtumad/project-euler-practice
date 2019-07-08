use std::env;
use std::str::FromStr;

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

// Finds the sum of multiples of 3 or 5 less than the input value
fn sum_multiples(max: i32) -> i32 {
    let result: i32 = (0..max)
        .filter(|x| (x % 3 == 0) || (x % 5 == 0))
        .sum();
    return result;
}

fn main() -> () {
    let max: i32 = get_arg(1);
    let result = sum_multiples(max);
    println!("Sum was {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_multiples_test() {
        use super::sum_multiples;
        assert_eq!(23, sum_multiples(10));
        assert_eq!(78, sum_multiples(20));
    }
}
