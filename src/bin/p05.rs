// This doesn't correspond to a problem, essentially just a template file

use std::env;
use std::str::FromStr;

#[allow(dead_code)]
fn get_arg<T: FromStr>(arg_num: usize) -> T {
    let args: Vec<String> = env::args().collect();
    if args.len() <= arg_num {
        panic!("Not enough arguments, expected at least {}", arg_num);
    }
    return match (&args[arg_num]).parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not parse argument: {}", &args[arg_num]),
    };
}

// returns a vector where the i_th element is the number of facors of i in the number
fn prime_factorize(mut number: usize, length: usize) -> Vec<u32> {
    let mut factors: Vec<u32> = vec![0; length];
    let mut current_factor = 2;
    while number > 1 {
        while number % current_factor == 0 {
            number = number / current_factor;
            factors[current_factor] += 1;
        }
        current_factor += 1;
    }
    return factors;
}

fn un_factorize(factors: &Vec<u32>) -> i64 {
    return factors
        .iter()
        .enumerate()
        .map(|(i, &count)| if i == 0 { 1 } else { i.pow(count) })
        .fold(1, |x, y| x * y) as i64;
}

// destructively sets values in f to max of the val in f and g
fn max_factors(f: &mut Vec<u32>, g: &Vec<u32>) -> () {
    assert_eq!(f.len(), g.len());
    for i in 0..g.len() {
        if g[i] > f[i] {
            f[i] = g[i];
        }
    }
}

// Finds the smallest number that is divisible by all of 1..max
fn main() -> () {
    let max: usize = get_arg::<usize>(1) + 1;
    let mut f: Vec<u32> = vec![0; max];
    let _: Vec<()> = (2..max)
        .map(|x| max_factors(&mut f, &prime_factorize(x, max)))
        .collect();
    let composite = un_factorize(&f);
    println!("{}", composite);
}

#[cfg(test)]
mod tests {
    #[test]
    fn un_factorize_test() {
        use super::un_factorize;
        assert_eq!(15, un_factorize(&vec![0, 0, 0, 1, 0, 1]));
        assert_eq!(90, un_factorize(&vec![0, 0, 1, 2, 0, 1]));
    }
    #[test]
    fn factorize_test() {
        use super::prime_factorize;
        let f = prime_factorize(15, 10);
        assert_eq!(1, f[3]);
        assert_eq!(0, f[4]);
        assert_eq!(0, f[2]);
        assert_eq!(1, f[5]);
        let g = prime_factorize(90, 6);
        assert_eq!(1, g[2]);
        assert_eq!(2, g[3]);
        assert_eq!(1, g[5]);
        assert_eq!(0, g[4]);
    }
}
