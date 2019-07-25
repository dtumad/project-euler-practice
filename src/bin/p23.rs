use project_euler_practice::prime::get_divisors;
#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn get_divisor_sum(n: u64) -> u64 {
    get_divisors(n).iter().sum()
}

struct DivisorCache {
    map: Vec<u64>,
}

impl DivisorCache {
    fn init(max: u64) -> DivisorCache {
        let mut map: Vec<u64> = Vec::with_capacity(2 * max as usize);
        map.push(0);
        for i in 1..=max {
            map.push(get_divisor_sum(i));
        }
        return DivisorCache { map };
    }

    fn get(&self, n: u64) -> u64 {
        let index = n as usize;
        if index < self.map.len() {
            self.map[n as usize]
        } else {
            get_divisor_sum(n)
        }
    }
}

fn is_abundant(dc: &DivisorCache, n: u64) -> bool {
    n < dc.get(n)
}

// whether n is the sum of two abundant numbers
fn is_abundant_sum(dc: &DivisorCache, n: u64) -> bool {
    if n < 24 {
        return false;
    }
    for i in 12..(n - 11) {
        if is_abundant(dc, i) & is_abundant(dc, n - i) {
            return true;
        }
    }
    return false;
}

fn get_non_abundant_sum_sum(max: u64) -> u64 {
    let dc = DivisorCache::init(max);
    return (1..=max).filter(|&n| !is_abundant_sum(&dc, n)).sum();
}

fn main() -> () {
    let result: u64 = get_non_abundant_sum_sum(get_arg_else(1, 28125));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn divisor_cache_test() {
        let dc = DivisorCache::init(10);
        assert_eq!(dc.get(1), 1);
        assert_eq!(dc.get(8), 7);
        assert_eq!(dc.get(6), 6);
        assert_eq!(dc.get(10), 8);
    }
    #[test]
    fn is_abundant_test() {
        let dc = DivisorCache::init(1000);
        assert!(is_abundant(&dc, 12));
        assert!(is_abundant(&dc, 24));
        assert!(is_abundant(&dc, 60));
        assert!(!is_abundant(&dc, 28));
        for i in 0..12 {
            assert!(!is_abundant(&dc, i));
        }
    }
    #[test]
    fn is_abundant_sum_test() {
        let dc = DivisorCache::init(1000);
        assert!(is_abundant_sum(&dc, 24));
        assert!(is_abundant_sum(&dc, 84));
        for i in 0..24 {
            assert!(!is_abundant_sum(&dc, i));
        }
    }
}
