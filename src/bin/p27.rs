use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

// evaluates n^2 + an + b
fn eval(n: i64, a: i64, b: i64) -> i64 {
    (n * n) + (a * n) + b
}

// gets the largest m such that n^2 + an + b is prime for all 0 <= n <= m
fn get_streak(a: i64, b: i64, pt: &mut PrimeTester) -> i64 {
    (0_i64..)
        .filter(|&n| {
            let p = eval(n, a, b);
            return (p <= 1) || !pt.is_prime(p as u64);
        })
        .next()
        .unwrap()
}

fn solve(a_max: i64, b_max: i64) -> (i64, i64) {
    let mut pt = PrimeTester::init((a_max * b_max) as usize);
    let mut max = 0;
    let mut max_source = (0, 0);
    for a in -a_max..=a_max {
        // if b is negative then the first value won't be prime
        for b in 2..=b_max {
            let s = get_streak(a, b, &mut pt);
            if s > max {
                max = s;
                max_source = (a, b);
            }
        }
    }
    return max_source;
}

fn main() -> () {
    let a_max = get_arg_else(1, 999);
    let b_max = get_arg_else(2, 1000);
    let result = solve(a_max, b_max);
    match result {
        (a, b) => println!("a = {}, b = {}, a*b = {}", a, b, a * b),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_streak_test() {
        use super::{get_streak, PrimeTester};
        let mut pt = PrimeTester::new();
        assert_eq!(get_streak(0, 0, &mut pt), 0);
        assert_eq!(get_streak(1, 2, &mut pt), 1);
        assert_eq!(get_streak(2, 2, &mut pt), 2);
        assert_eq!(get_streak(-4, 2, &mut pt), 1);
        assert_eq!(get_streak(1, 41, &mut pt), 40);
        assert_eq!(get_streak(-79, 1601, &mut pt), 80);
    }
}
