#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn solve(size: usize) -> u64 {
    (2..)
        .filter(|&n| n % 2 == 0)
        .map(|n| {
            std::iter::repeat(n)
                .take(4)
        })
        .flatten()
        .take(2 * size - 2)
        .fold(vec![1], |mut acc, diff: u64| {
            acc.push(acc[acc.len() - 1] + diff);
            acc
        })
        .iter()
        .sum()
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(3), 25);
        assert_eq!(solve(5), 101);
    }
}
