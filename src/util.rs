use std::env;
use std::str::FromStr;

/// Gets the specified command line argument, indexed from 0.
/// Note that most command lines pass argument 0 automatically as the file name.
pub fn get_arg<T: FromStr>(arg_num: usize) -> T {
    let args: Vec<String> = env::args().collect();
    if args.len() <= arg_num {
        panic!("Not enough arguments, expected at least {}", arg_num);
    }
    return match (&args[arg_num]).parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not parse argument: {}", &args[arg_num]),
    };
}

/// Equivalent to get_arg, but returns the given default rather than panicking if arg not given
/// Still panics if an arg is given but can't be parsed proberly
///
/// ```
/// use project_euler_practice::util::get_arg_else;
/// assert_eq!(get_arg_else(5, 123_u64), 123_u64);
/// ```
pub fn get_arg_else<T: FromStr>(arg_num: usize, default: T) -> T {
   let args: Vec<String> = env::args().collect();
    if args.len() <= arg_num {
        return default;
    }
    return match (&args[arg_num]).parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not parse argument: {}", &args[arg_num]),
    };

}

use std::fs;
/// Gets the specified file as a string
///
/// ```
/// use project_euler_practice::util::read_input;
/// assert_eq!(read_input("test").trim(), "Hello World")
/// ```
pub fn read_input(file_name: &str) -> String {
    let mut path = "./src/inputs/".to_owned();
    path.push_str(file_name);
    path.push_str(".txt");
    return fs::read_to_string(path.clone())
        .expect(&format!("Something went wrong reading {}", path.clone()));
}
