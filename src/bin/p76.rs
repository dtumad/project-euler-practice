#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn solve(mut n: usize) -> u64 {
    n += 1;
    // ways_to_add[i][j] is ways to add to i using values up to j
    let def: Vec<u64> = vec![0; n];
    let mut ways_to_add: Vec<Vec<u64>> = vec![def; n];
    ways_to_add[0] = vec![1; n];
    ways_to_add[1] = vec![1; n];
    for i in 2..n {
        for j in 1..n {
            ways_to_add[i][j] = (1..=std::cmp::min(i, j))
                .map(|m| ways_to_add[i - m][m])
                .sum()
        }
    }
    return ways_to_add[n - 1][n - 2];
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solution_test() {
        use super::solve;
        assert_eq!(solve(5), 6);
        assert_eq!(solve(1), 1);
    }
}
