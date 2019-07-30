#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

// gets the index of the first fibinocii number at least as large as min
fn get_fib(min: BigUint) -> usize {
    let mut trail: BigUint = One::one(); 
    let mut lead: BigUint = One::one(); 
    std::iter::from_fn(move || {
        let old_trail = trail.clone();
        let new_lead = &trail + &lead;
        trail = lead.clone();
        lead = new_lead;
        Some(old_trail)
    }).enumerate()
    .filter(|(_, f)| f >= &min)
    .map(|(index, _)| index)
    .next()
    .unwrap() + 1
}

// gets the smallest biguint with specified number of digits
fn get_min(digits: u64) -> BigUint {
    let mut result = 1.to_biguint().unwrap();
    let mut current_digits = 1;
    while current_digits < digits {
        result *= 10.to_biguint().unwrap();
        current_digits += 1;
    }
    return result;
}


fn main() -> () {
    let result: usize = get_fib(get_min(get_arg(1)));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_fib_test() {
        use super::get_fib;
        use num_bigint::ToBigUint;
        assert_eq!(get_fib(5.to_biguint().unwrap()), 5);
        assert_eq!(get_fib(100.to_biguint().unwrap()), 12);
    }
    #[test]
    fn get_min_test() {
        use super::get_min;
        use num_bigint::ToBigUint;
        assert_eq!(get_min(3), 100.to_biguint().unwrap());
        assert_eq!(get_min(6), 100_000.to_biguint().unwrap());
    }
}
