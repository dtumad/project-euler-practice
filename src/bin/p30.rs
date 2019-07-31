#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

// gets the ith digit of n, going right to left
fn get_digit(d: u64, i: u32) -> usize {
    ((d / (10_u64.pow(i))) % 10) as usize
}

// number of digits in n
fn get_digit_count(n: u64) -> u32 {
    ((n as f64).log10() + 1.) as u32
}

// sums the pth power of each digit in n
fn get_digit_sum(n: u64, power_cache: &Vec<u64>) -> u64 {
    (0..get_digit_count(n))
        .map(|i| power_cache[get_digit(n, i)])
        .sum()
}

// we only need to check up to 9^p * (p+1)
fn get_search_max(p: u32) -> u64 {
    9_u64.pow(p) * (p as u64 + 1)
}

fn get_power_cache(p: u32) -> Vec<u64> {
    (0..10_u64).map(|n| n.pow(p)).collect()
}

// returns the sum of all numbers whose digit sum is the original
fn sum_digit_idempotent_numbers(p: u32) -> u64 {
    let power_cache = get_power_cache(p);
    (2..get_search_max(p))
        .filter(|&n| n == get_digit_sum(n, &power_cache))
        .sum()
}

fn main() -> () {
    let result: u64 = sum_digit_idempotent_numbers(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn digit_test() {
        use super::get_power_cache;
        let pc = get_power_cache(2);
        assert_eq!(pc[3], 9);
        assert_eq!(pc[9], 81);
        assert_eq!(pc[0], 0);

        use super::{get_digit, get_digit_count, get_digit_sum};
        let n = 9876543210;
        for i in 0..10 {
            assert_eq!(get_digit(n, i), i as usize);
        }
        assert_eq!(get_digit_count(n), 10);
        assert_eq!(get_digit_sum(n, &get_power_cache(1)), 45);
        assert_eq!(get_digit_sum(1634, &get_power_cache(4)), 1634);
    }
    #[test]
    fn idempotent_sum_test() {
        use super::sum_digit_idempotent_numbers as sdin;
        assert_eq!(sdin(4), 19316);
    }
}
