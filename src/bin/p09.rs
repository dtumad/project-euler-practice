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

fn is_pyth_triple(a: u64, b: u64, c: u64) -> bool {
    return (a * a) + (b * b) == (c * c);
}

// returns abc, where a,b,c are the first pyth triple with the given sum
fn find_triple(sum: u64) -> Option<u64> {
    for c in 1..sum {
        let max_a;
        if 2 * c > sum {
            max_a = sum - c;
        } else {
            max_a = c;
        }

        for a in 1..max_a {
            let b = sum - c - a;
            if is_pyth_triple(a, b, c) {
                return Some(a * b * c);
            }
        }
    }
    return None;
}

// Doesn't do much
fn main() -> () {
    println!("{:?}", find_triple(get_arg(1)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_triple_test() {
        use super::find_triple;
        assert_eq!(find_triple(6), None);
        assert_eq!(find_triple(12), Some(60));
    }
}
