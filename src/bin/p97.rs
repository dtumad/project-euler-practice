#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

// computes the last d digits of (m * (2 ^ p)) + 1
fn solve(d: u32, mut m: u64, mut p: u64) -> u64 {
    let modulus = 10_u64.pow(d);
    m %= modulus;
    while p > 0 {
        m *= 2;
        m %= modulus;
        p -= 1;
    }
    return m + 1;
}

fn main() -> () {
    let result = solve(get_arg_else(1, 10), get_arg_else(2, 28433), get_arg_else(3, 7_830_457));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(4, 1, 20), 8577);
        assert_eq!(solve(3, 2, 20), 153);
    }
}
