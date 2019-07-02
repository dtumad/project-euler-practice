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
fn main() -> () {
    let max: i32 = get_arg(1);
    let result: i32 = (0..max)
        .filter(|x| (x % 3 == 0) || (x % 5 == 0))
        .sum();
    println!("Sum was {}", result);
}
