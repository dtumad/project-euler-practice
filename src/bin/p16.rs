#[allow(unused_imports)]
use project_euler_practice::util::get_arg;

extern crate num_bigint;
use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn digit_sum(base: u64, exponent: u64) -> u32 {
    let base: BigInt = FromPrimitive::from_u64(base).unwrap();
    num_traits::pow(base, exponent as usize)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn main() -> () {
    let result: u32 = digit_sum(get_arg(1), get_arg(2));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn digit_sum_test() {
        assert_eq!(super::digit_sum(2, 15), 26);
        assert_eq!(super::digit_sum(2, 4), 7);
    }
}
