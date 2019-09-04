#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn get_sub_nums(file: &str) -> Vec<u64> {
    read_input(file)
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn is_sub_num(mut n: u64, mut sub: u64) -> bool {
    while n > 0 {
        if n % 10 == sub % 10 {
            sub /= 10;
        }
        n /= 10;
    }
    return sub == 0;
}

fn solve(file: String) -> u64 {
    let sub_nums = get_sub_nums(&file);
    (1..)
        .filter(|&n| {
            sub_nums.iter()
                .all(|&sub| is_sub_num(n, sub))
        })
        .next()
        .unwrap()
}

fn main() -> () {
    let result = solve(get_arg_else(1, "p79".to_string()));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sub_num_test() {
        use super::is_sub_num;
        assert!(is_sub_num(123456, 135));
        assert!(is_sub_num(123456, 146));
        assert!(is_sub_num(123456, 23456));
        assert!(is_sub_num(123456, 123456));
        assert!(is_sub_num(123456, 12));
        assert!(is_sub_num(123456, 16));


        assert!(!is_sub_num(123456, 1435));
        assert!(!is_sub_num(123456, 147));
        assert!(!is_sub_num(123456, 223456));
        assert!(!is_sub_num(123456, 1234560));
        assert!(!is_sub_num(123456, 120));
        assert!(!is_sub_num(123456, 166));
    }
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve("p79_test".to_string()), 123456);
    }
}
