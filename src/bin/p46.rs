#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::PrimeTester;
#[allow(unused_imports)]
use rayon::prelude::*;

fn satisfies_conjecture(n: u64, pt: &mut PrimeTester) -> bool {
    if n % 2 == 0 || pt.is_prime(n) {
        return true;
    }
    let mut i = 1;
    loop {
        let j = 2 * i * i;
        if j > n {
            return false;
        }
        if pt.is_prime(n - j) {
            return true;
        }
        i += 1;
    }
}

fn solve() -> u64 {
    let mut pt = PrimeTester::new();
    (2..)
        .filter(|&n| !satisfies_conjecture(n, &mut pt))
        .next()
        .unwrap()
}

fn main() -> () {
    let result = solve();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn satisfies_conjecture_test() {
        use super::{satisfies_conjecture, PrimeTester};
        let mut pt = PrimeTester::new();
        for i in 2..1000 {
            println!("{}", i);
            assert!(satisfies_conjecture(i, &mut pt));
        }
    }
}
