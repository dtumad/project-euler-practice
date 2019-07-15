#[allow(unused_imports)]
use project_euler_practice::util::get_arg;

// solution is always 2n choose n
fn lattice_paths(n: u64) -> u64 {
   // fac(2 * n) / fac(n).pow(2)
   (1..=n)
       .map(|m| (n as f64 + m as f64) / m as f64)
       .product::<f64>() as u64
}

fn main() -> () {
    println!("{}", lattice_paths(get_arg(1)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn lattice_test() {
        use super::{lattice_paths};
        assert_eq!(lattice_paths(2), 6);
        assert_eq!(lattice_paths(3), 20);
    }
}
