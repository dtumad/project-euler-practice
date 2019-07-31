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

// sums the factorial of each digit in n
fn get_digit_sum(n: u64, factorial_cache: &Vec<u64>) -> u64 {
    (0..get_digit_count(n))
        .map(|i| factorial_cache[get_digit(n, i)])
        .sum()
}

fn get_factorial_cache() -> Vec<u64> {
    (0..10_u64).map(|n| (1..=n).product()).collect()
}

// returns the sum of all numbers whose digit sum is the original
fn sum_digit_idempotent_numbers(search_max: u64) -> u64 {
    let factorial_cache = get_factorial_cache();
    (3..=search_max)
        .filter(|&n| n == get_digit_sum(n, &factorial_cache))
        .sum()
}

fn main() -> () {
    let search_max = get_arg_else(1, 300_000);
    let result: u64 = sum_digit_idempotent_numbers(search_max);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn digit_test() {
        use super::get_factorial_cache;
        let pc = get_factorial_cache();
        assert_eq!(pc[3], 6);
        assert_eq!(pc[4], 24);
        assert_eq!(pc[5], 120);
        assert_eq!(pc[9], 362880);
        assert_eq!(pc[8], 40320);

        use super::{get_digit, get_digit_count, get_digit_sum};
        let n = 9876543210;
        for i in 0..10 {
            assert_eq!(get_digit(n, i), i as usize);
        }
        assert_eq!(get_digit_count(n), 10);

        let fc = get_factorial_cache();
        assert_eq!(get_digit_sum(145, &fc), 145);
        assert_eq!(get_digit_sum(1020304050, &fc), 158);
        assert_eq!(get_digit_sum(10030005, &fc), 132);
        assert_eq!(get_digit_sum(40585, &fc), 40585);
    }
}
