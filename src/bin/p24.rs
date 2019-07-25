#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn fac(n: u8) -> u64 {
    (1..=(n as u64)).product()
}

#[derive(Debug, PartialEq, Eq)]
struct Permutation {
    // should contain only one of each number 0-n
    perm: Vec<u8>,
}

impl Permutation {
    fn to_string(&self) -> String {
        self.perm
            .iter()
            .map(|i| i.to_string())
            .fold(String::from(""), |s, i| s + &i)
    }

    // adds the given number to the front, and increments other indices as needed
    fn prepend_with(&self, p: u8) -> Permutation {
        let mut perm = vec![p];
        perm.extend(self.perm.iter().map(|&q| if q < p { q } else { q + 1 }));
        return Permutation { perm };
    }

    // gets the mth permutation of 0..n in order
    fn get_permutation(n: u8, m: u64) -> Permutation {
        // check bounds on m: 0 < m < n!
        //assert!((m > 0) & (m  <= ((1..=(n+1)).product())));
        if n == 0 {
            Permutation{perm: vec![0]}
        }
        else {
            let prod = fac(n);
            let prefix = m / prod;
            let suffix = m % prod;
            Self::get_permutation(n-1, suffix).prepend_with(prefix as u8)
        }
    }

    // gets all permuations of 0..n in order
    #[allow(dead_code)]
    fn get_permutations(n: u8) -> Vec<Permutation> {
        if n == 0 {
            let zero_perm = Permutation{perm: vec![0]};
            return vec![zero_perm];
        } else {
            let sub_perms = Self::get_permutations(n - 1);
            let perm_count = fac(n) as usize;
            let mut result = Vec::with_capacity(perm_count * 2);
            for prefix in 0..=n {
                result.extend(
                    sub_perms.iter()
                    .map(|permutation| permutation.prepend_with(prefix))
                    .collect::<Vec<Permutation>>());
            }
            return result;
        }
    }
}

use std::fmt;
impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn main() -> () {
    let how_many = get_arg(1);
    let which_one: u64 = get_arg(2);
    let result = Permutation::get_permutation(how_many, which_one - 1);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn permutation_test() {
        use super::*;
        let p = Permutation {
            perm: vec![1, 2, 0],
        };
        let q = Permutation {
            perm: vec![3, 1, 0, 2],
        };
        assert_eq!(p.to_string(), String::from("120"));
        assert_eq!(q.to_string(), "3102");
        let r = q.prepend_with(1);
        assert_eq!(r.to_string(), "14203");

        let perms3 = Permutation::get_permutations(2);
        assert_eq!(perms3[0].to_string(), "012");
        assert_eq!(perms3[1].to_string(), "021");
        assert_eq!(perms3[5].to_string(), "210");

        for i in 0..6 {
            assert_eq!(perms3[i as usize], Permutation::get_permutation(2, i));
        }
    }
}
