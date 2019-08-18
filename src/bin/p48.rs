#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

// last n digits of the number
fn last_n_digits(num: u64, n: u64) -> u64 {
    num % (10_u64.pow(n as u32))
}

// last 10 digits of num^num
fn self_power(num: u64) -> u64 {
    let mut pow = num;
    let mut result = 1;
    while pow > 0 {
        result *= num;
        pow -= 1;
        result = last_n_digits(result, 10);
    }
    return result;
}

fn solve(n: u64) -> u64 {
    last_n_digits((1..=n).map(|m| self_power(m)).sum(), 10)
}

fn main() -> () {
    let result: u64 = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn self_power_test() {
        use super::last_n_digits;
        assert_eq!(last_n_digits(123456789, 3), 789);
        assert_eq!(last_n_digits(123454321, 7), 3454321);
        use super::self_power;
        assert_eq!(self_power(100), 0);
        assert_eq!(self_power(2), 4);
        assert_eq!(self_power(3), 27);
        assert_eq!(self_power(12), 6100448256);
        assert_eq!(self_power(15), 380859375);
    }
}
