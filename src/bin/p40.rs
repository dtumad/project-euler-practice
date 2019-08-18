#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn len(n: u64) -> u64 {
    ((n as f64).log10()) as u64 + 1
}

fn champernowne(mut n: u64) -> u64 {
    let mut champ = 1;
    while len(champ) < n {
        n -= len(champ);
        champ += 1;
    }
    n = len(champ) - n + 1;
    while n > 1 {
        n -= 1;
        champ /= 10;
    }
    return champ % 10;
}

fn solve(n: u64) -> u64 {
    (0..=n)
        .map(|p| 10_u64.pow(p as u32))
        .map(|t| champernowne(t))
        .product()
}

fn main() -> () {
    let result: u64 = solve(get_arg_else(1, 6));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn champernowne_test() {
        use super::{len, champernowne as c};
        for i in 1..10 {
            assert_eq!(len(i), 1);
            assert_eq!(c(i), i);
        }
        for i in 10..100 {
            assert_eq!(len(i), 2);
        }
        assert_eq!(len(100), 3);
        assert_eq!(c(12), 1);
        assert_eq!(c(14), 1);
        assert_eq!(c(15), 2);
        assert_eq!(c(19), 4);
    }
}
