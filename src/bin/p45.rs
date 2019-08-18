#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use std::collections::HashSet;
use std::hash::Hash;

// only works for monotonic functions
struct FormulaCache<T: Ord + Hash + Copy> {
    cache: HashSet<T>,
    last_in: u64,
    last_out: T,
    formula: Box<Fn(u64) -> T>,
}

impl<T: Ord + Hash + Copy> FormulaCache<T> {
    fn new(formula: Box<Fn(u64) -> T>) -> FormulaCache<T> {
        let mut hs = HashSet::new();
        // initialize with some values
        let mut o = formula(1);
        for i in 1..10 {
            o = formula(i);
            hs.insert(o);
        }
        return FormulaCache {
            formula: formula,
            cache: hs,
            last_in: 9,
            last_out: o,
        };
    }

    fn grow_to(&mut self, out: T) -> () {
        while out > self.last_out {
            let mut out_i = (self.formula)(self.last_in);
            for i in self.last_in..=(self.last_in * 2) {
                out_i = (self.formula)(i);
                self.cache.insert(out_i);
            }
            self.last_in *= 2;
            self.last_out = out_i;
        }
    }

    fn contains(&mut self, out: T) -> bool {
        self.grow_to(out);
        return self.cache.contains(&out);
    }
}

fn solve(i: usize) -> u64 {
    let mut pent = FormulaCache::new(Box::new(|n| n * (3 * n - 1) / 2));
    //let mut hex = FormulaCache::new(Box::new(|n| n * (2 * n - 1)));
    (1..)
        .map(|n| n * (2 * n - 1))
        .filter(|&n| pent.contains(n)) // && hex.contains(n))
        .nth(i)
        .unwrap()
}

fn main() -> () {
    let result: u64 = solve(get_arg_else(1, 2));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn formula_cahce_test() {
        use super::FormulaCache;
        let mut tri = FormulaCache::new(Box::new(|n| n * (n + 1) / 2));
        for i in 1..100 {
            let t = i * (i + 1) / 2;
            assert!(tri.contains(t));
            assert!(!tri.contains(t + 1));
        }
    }
}
