#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn count_integer_triangles(p: u64) -> u8 {
    let max_side = p / 2;
    let mut count = 0;
    for a in 1..=max_side {
        let max_b = (p - a) / 2;
        for b in a..=max_b {
            if (p - a - b).pow(2) == a * a + b * b {
                count += 1;
            }
        }
    }
    return count;
}

fn solve(n: u64) -> u64 {
    (1..=n)
        .into_par_iter()
        .filter(|&m| m % 2 == 0)
        .map(|m| (count_integer_triangles(m), m))
        .max()
        .unwrap()
        .1
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_integer_triangles_test() {
        use super::count_integer_triangles;
        assert_eq!(count_integer_triangles(120), 3);
        assert_eq!(count_integer_triangles(12), 1);
    }
}
