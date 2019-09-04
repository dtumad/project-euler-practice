#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

fn get_chain_terminal(mut n: u64) -> u64 {
    loop {
        let mut m = 0;
        while n > 0 {
            m += (n % 10).pow(2);
            n /= 10;
        }
        if m == 89 || m == 1 {
            return m;
        }
        n = m;
    }
}

fn solve(n: u64, terminal: u64) -> u64 {
    (1..=n)
        .filter(|&m| get_chain_terminal(m) == terminal)
        .count() as u64
}

fn main() -> () {
    let result = solve(get_arg(1), get_arg_else(2, 89));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_chain_terminal_test() {
        use super::get_chain_terminal;
        assert_eq!(get_chain_terminal(89), 89);
        assert_eq!(get_chain_terminal(1), 1);
        assert_eq!(get_chain_terminal(44), 1);
        assert_eq!(get_chain_terminal(85), 89)
    }
}
