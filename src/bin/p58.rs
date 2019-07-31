use project_euler_practice::prime::is_prime;
#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

// returns true if the number of primes in the spiral is below the given density
fn check_density(density: f64) -> u64 {
    let mut prime_count = 0;
    let mut total_count = 1;
    let mut current = 1;
    let diffs = (2..)
        .filter(|&n| n % 2 == 0)
        .map(|n| std::iter::repeat(n).take(4))
        .flatten();
    for diff in diffs {
        current += diff;
        total_count += 1;
        if is_prime(current) {
            prime_count += 1
        }
        if (prime_count as f64) / (total_count as f64) <= density && (total_count - 1) % 4 == 0 {
            break;
        }
    }
    return ((total_count - 1) / 2) + 1;
}

fn main() -> () {
    let result = check_density(get_arg(1));
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solution_test() {
        use super::check_density;
        assert_eq!(check_density(0.63), 3);
        assert_eq!(check_density(0.50), 11);
    }
}
