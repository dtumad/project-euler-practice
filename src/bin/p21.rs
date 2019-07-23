#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
use project_euler_practice::prime::get_divisors;

fn get_divisor_sum(n: u64) -> u64 {
    get_divisors(n).iter()
        .sum()
}

struct DivisorCache {
    map: Vec<u64>
}

impl DivisorCache {
    fn init(max: u64) -> DivisorCache {
        let mut map: Vec<u64> = Vec::with_capacity(2 * max as usize);
        map.push(0);
        for i in 1..=max {
            map.push(get_divisor_sum(i));
        }
        return DivisorCache{map};
    }

    fn get(&self, n: u64) -> u64 {
        let index = n as usize;
        if index < self.map.len() {
            self.map[n as usize]
        }
        else {
            get_divisor_sum(n)
        }
    }
}

fn is_amicable(dc: &DivisorCache, n: u64) -> bool {
    let div_n = dc.get(n);
    (div_n != n) & (dc.get(div_n) == n)
}

fn get_amicable_sum(max: u64) -> u64 {
    let dc = DivisorCache::init(max);
    return (1..=max)
        .filter(|&n| is_amicable(&dc, n))
        .sum();
}

fn main() -> () {
    let result: u64 = get_amicable_sum(get_arg_else(1, 10000));
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
    fn is_amicable_test() {
        let dc = DivisorCache::init(1000);
        assert!(is_amicable(&dc, 220));
        assert!(is_amicable(&dc, 284));
        assert_eq!(is_amicable(&dc, 1), false);
        assert_eq!(is_amicable(&dc, 6), false);
        assert_eq!(is_amicable(&dc, 10), false);
    }
}
