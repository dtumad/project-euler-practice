#[derive(Debug, PartialEq, Clone)]
pub struct BigNum {
    // low order slots are low order digits. sum (digits[n] * 10^n)
    digits: Vec<u8>,
}

/// Represents a large number that couldn't fit in for example a i128
///
/// Converting to this format involves reversing the order of digits
/// Also clears out leading zeros that may exist
///
/// ```
/// use project_euler_practice::big_num::BigNum;
/// assert_eq!(BigNum::from_string("0123456").to_string(), "123456");
/// assert_eq!(BigNum::from_string("000000").to_string(), "0");
/// ```
impl BigNum {
    pub fn from_string(number: &str) -> Self {
        let mut result = Self {
            digits: number
                .chars()
                .rev()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        };
        result.clear_lead_zeros();
        return result;
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("");
        for d in self.digits.iter().rev() {
            result.push_str(&d.to_string());
        }
        return result;
    }

    fn clear_lead_zeros(&mut self) -> () {
        while self.digits.len() > 1 && self.digits[self.digits.len() - 1] == 0 {
            self.digits.pop();
        }
    }

    fn len(&self) -> usize {
        self.digits.len()
    }
}

use std::fmt;
impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in self.digits.iter().rev() {
            write!(f, "{}", d).ok();
        }
        write!(f, "")
    }
}

/// Adition operations will never overflow
///
/// ```
/// use project_euler_practice::big_num::BigNum;
/// let a = BigNum::from_string("123");
/// let b = BigNum::from_string("456");
/// let d = BigNum::from_string("456");
/// let c = BigNum::from_string("789");
/// assert_eq!((a+b).to_string(), "579");
/// assert_eq!((d+c).to_string(), "1245");
/// ```
impl std::ops::Add for BigNum {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let new_size = std::cmp::max(self.len(), other.len()) + 1;
        let mut new_digits = vec![0; new_size];
        for (i, &d) in self.digits.iter().enumerate() {
            new_digits[i] += d;
        }
        for (j, &d) in other.digits.iter().enumerate() {
            new_digits[j] += d;
            // we can assume we only carry a one, never any more
            if new_digits[j] > 9 {
                new_digits[j + 1] += 1;
                new_digits[j] -= 10;
            }
        }
        let mut result = Self { digits: new_digits };
        result.clear_lead_zeros();
        return result;
    }
}
