#[allow(unused_imports)]
use project_euler_practice::util::get_arg;
use std::collections::HashMap;

fn next_collatz(n: u64) -> u64 {
    match n % 2 {
        0 => n / 2,
        1 => 3 * n + 1,
        _ => panic!("Can't tell if n is even or odd"),
    }
}

// cache is used to memoize the values
fn get_collatz_length(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    match cache.get(&n) {
        Some(&l) => l,
        None => {
            let l = 1 + get_collatz_length(next_collatz(n), cache);
            cache.insert(n, l);
            l
        }
    }
}

fn main() -> () {
    let mut cache = HashMap::new();
    cache.insert(1, 1);

    let max = get_arg(1);
    let result = (max / 2..max)
        .map(|n| (get_collatz_length(n, &mut cache), n))
        .max()
        .unwrap();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn next_collatz_test() {
        use super::next_collatz;
        assert_eq!(next_collatz(8), 4);
        assert_eq!(next_collatz(7), 22);
    }
    #[test]
    fn get_collatz_length_test() {
        use super::get_collatz_length;
        let mut cache = std::collections::HashMap::new();
        cache.insert(1, 1);
        assert_eq!(get_collatz_length(13, &mut cache), 10);
    }
}
