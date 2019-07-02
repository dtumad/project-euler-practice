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

// Doesn't do much
fn main() -> () {
    println!("I'm just a template");
}
