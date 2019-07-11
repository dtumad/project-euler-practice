use project_euler_practice::prime::get_num_divisors;
use project_euler_practice::util::get_arg;

// gets the first triangle number with at least the input number of divisors
fn solve(desired_num_divisors: u64) -> u64 {
    let mut triangle = 1;
    let mut next_jump = 2;
    return std::iter::from_fn(move || {
        triangle += next_jump;
        next_jump += 1;
        Some(triangle)
    })
    .filter(|&n| get_num_divisors(n) > desired_num_divisors)
    .nth(0)
    .unwrap();
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(1), 3);
        assert_eq!(solve(3), 6);
        assert_eq!(solve(4), 28);
        assert_eq!(solve(5), 28);
    }
}
