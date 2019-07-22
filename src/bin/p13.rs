#[allow(unused_imports)]
use project_euler_practice::util::get_arg_else;
use project_euler_practice::util::read_input;

extern crate num_bigint;
extern crate num_traits;
use num_bigint::BigUint;
use num_traits::Zero;

fn get_inputs(file: &str) -> Vec<BigUint> {
    read_input(file)
        .lines()
        .map(|s| s.trim().parse::<BigUint>().unwrap())
        .collect()
}

// Sums the first n of the given numbers, n being the input
fn main() -> () {
    let inputs: Vec<BigUint> = get_inputs(&get_arg_else(1, "p13".to_owned()));
    let mut sum: BigUint = Zero::zero();
    for n in inputs.iter().take(get_arg_else(2, 100)) {
        sum = sum + n;
    }
    println!("{}", sum);
}
