#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::count_distinct_prime_divisors as f;

// first number m such that m..m+n all have n distinct prime factors
fn solve(n: u64) -> u64 {
    let mut poss_sol = 0;
    let mut streak = 0;
    for m in 2.. {
        if f(m) == n {
            if streak == 0 {
                poss_sol = m;
            }
            streak += 1;
        }
        else {
            streak = 0;
        }
        
        if streak == n {
            break;
        }
    }
    return poss_sol;
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
        assert_eq!(solve(2), 14);
        assert_eq!(solve(3), 644);
    }
}
