use std::env;
use std::str::FromStr;

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

use std::fs;

/// Gets the specified file as a string
///
/// ``` 
/// use project_euler_practice::util::read_input;
/// assert_eq!(read_input("test"), "Hello World\n")
/// ```
pub fn read_input(file_name: &str) -> String {
    let mut path = "./src/inputs/".to_owned();
    path.push_str(file_name);
    path.push_str(".txt");
    return fs::read_to_string(path.clone())
        .expect(&format!("Something went wrong reading {}", path.clone()));
}
