#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;
use std::collections::HashSet;

// contains a collection of 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Purse {
    coins: [u8; 8],
}
impl Purse {
    fn zero_purse() -> Purse {
        Purse { coins: [0; 8] }
    }
    // adds one coin with the given value
    fn add_coin(&self, value: usize) -> Purse {
        let mut new_purse = self.clone();
        new_purse.coins[match value {
            1 => 1,
            2 => 2,
            5 => 3,
            10 => 4,
            20 => 5,
            50 => 6,
            100 => 7,
            200 => 8,
            _ => panic!("Invalid coin value {}", value),
        } - 1 as usize] += 1;
        return new_purse;
    }
}

type PurseCache = Vec<Option<HashSet<Purse>>>;

fn copy_set(s: &HashSet<Purse>) -> HashSet<Purse> {
    let mut result = HashSet::new();
    for &purse in s.iter() {
        result.insert(purse);
    }
    return result;
}

fn get_purse_set(total: usize, purse_cache: &mut PurseCache) -> HashSet<Purse> {
    if let Some(s) = &purse_cache[total] {
        return copy_set(s);
    } else {
        let coins: Vec<usize> = vec![1, 2, 5, 10, 20, 50, 100, 200];
        let mut result = HashSet::new();
        for &c in coins.iter() {
            if total < c {
                continue;
            }
            let sub_purse_set = get_purse_set(total - c, purse_cache);
            for purse in sub_purse_set.iter() {
                result.insert(purse.add_coin(c));
            }
        }
        purse_cache[total] = Some(copy_set(&result));
        return result;
    }
}

fn solve(n: usize) -> u64 {
    let mut purse_cache: PurseCache = vec![None; n + 1];
    let mut zero_set = HashSet::new();
    zero_set.insert(Purse::zero_purse());
    purse_cache[0] = Some(zero_set);
    let result = get_purse_set(n, &mut purse_cache);
    return result.iter().len() as u64;
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
        assert_eq!(solve(5), 4);
        assert_eq!(solve(10), 11);
    }
}
