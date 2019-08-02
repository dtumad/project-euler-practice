#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn is_pandigital(n: &str) -> bool {
    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    n.len() == 9 &&
        digits.iter()
            .fold(true, |acc, &digit| acc && n.contains(digit))
}

const MIN: &str = "000000000";
fn solve(max_n: u64) -> u64 {
    let mut max = MIN.to_owned();
    for i in 2..max_n {
        let max_for_i = (1..=5)
            .map(|n| (1..=n)
                 .map(|m| i * m)
                 .fold("".to_owned(), |acc, n| format!("{}{}", acc, n.to_string()))     
            )
            .filter(|s| is_pandigital(s))
            .max()
            .unwrap_or(MIN.to_owned());
        max = std::cmp::max(max, max_for_i);
    }
    return max.parse::<u64>().unwrap();
}

fn main() -> () {
    let result: u64 = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_pandigital_test() {
        use super::is_pandigital;
        assert!(is_pandigital("123456789"));
        assert!(is_pandigital("928174356"));
        assert!(!is_pandigital("1234567890"));
        assert!(!is_pandigital("234567890"));
        assert!(!is_pandigital("123455789"));
        assert!(!is_pandigital("193746555"));
    }
}
