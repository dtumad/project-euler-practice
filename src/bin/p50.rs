use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use std::collections::HashSet;

fn solve(max: u64) -> u64 {
    let primes = PrimeTester::init(max as usize).get_primes_ord(max as usize);
    let prime_set: HashSet<u64> = primes.iter().fold(HashSet::new(), |mut acc, &e| {
        acc.insert(e);
        acc
    });
    let mut result = (primes[0], 1); // prime, primes in sum
    for i in 0..primes.len() {
        if (primes.len() - 7) < result.1 {
            break;
        }
        let mut current = (primes[i], 1);
        let mut j = i + 1;
        while current.0 < max && j < primes.len() {
            current = (current.0 + primes[j], current.1 + 1);
            if current.1 > result.1 && prime_set.contains(&current.0) {
                result = current;
            }
            j += 1;
        }
    }
    return result.0;
}

fn main() -> () {
    let result: u64 = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(100), 41);
        assert_eq!(solve(1000), 953);
    }
}
