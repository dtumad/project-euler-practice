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

// Finds the larges prime factor of the given input
fn main() -> () {
    let mut to_factor: i64 = get_arg(1);
    let to_factor_cache = to_factor;
    
    // first check for powers of two, allows faster counting later
    while to_factor % 2 == 0 {
        to_factor = to_factor / 2;
    }
    if to_factor == 1 {
        println!("The largest prime factor of {} is {}",
                 to_factor_cache, 2);
        return;
    }
                 
    // otherwise count up through the rest of the odd numbers
    let mut largest_factor = 3;
    while to_factor > 1 {
        while to_factor % largest_factor == 0 {
            to_factor = to_factor / largest_factor;
        }
        // this overcounts exactly once on the last iteration
        largest_factor += 2;
    }

    println!("The larges prime factor of {} is {}",
             to_factor_cache, largest_factor - 2);
}
