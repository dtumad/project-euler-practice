use std::collections::HashSet;

/// returns if the given number is prime
///
/// This version doesn't cache primes, so is inefficient over multiple calls
///
/// ```
/// use project_euler_practice::prime::is_prime;
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(17), true);
/// assert_eq!(is_prime(25), false);
/// assert_eq!(is_prime(12512345122), false);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        return n == 2 || n == 3;
    }
    let max = (((n as f64).sqrt()) as u64) + 1;
    for d in 5..max {
        if n % d == 0 {
            return false;
        }
    }
    return true;
}

/// creates a vector where entry i is the power of i in the prime factorization of n
///
/// ```
/// use project_euler_practice::prime::prime_factorize;
/// assert_eq!(prime_factorize(6), vec![0,0,1,1,0,0,0]);
/// assert_eq!(prime_factorize(5), vec![0,0,0,0,0,1]);
/// ```
pub fn prime_factorize(n: u64) -> Vec<u8> {
    let mut prime_factorization: Vec<u8> = vec![0; n as usize + 1];
    let mut d = 2;
    let mut m = n as usize;
    while m > 1 {
        while m % d == 0 {
            m = m / d;
            prime_factorization[d] += 1;
        }
        d = d + 1;
    }
    return prime_factorization;
}

/// gets a hash set containing all the divisors about n
/// doesn't guaruntee anything about the ordering
///
/// ```
/// use project_euler_practice::prime::get_divisors as f;
/// let divs = f(630);
/// assert_eq!(divs.len(), 24);
/// for i in 1..=630 {
///     if 630 % i == 0 {
///         assert!(divs.contains(&i));
///     }
/// }
/// ```
pub fn get_divisors(n: u64) -> HashSet<u64> {
    let mut divisors = HashSet::with_capacity(get_num_divisors(n) as usize);
    let max = (n as f64).sqrt() as u64;
    for i in 1..=max {
        if n % i == 0 {
            divisors.insert(i);
            divisors.insert(n / i);
        }
    }
    return divisors;
}

/// gets the number of divisors of the given integer
/// more efficient than counting the result of get_divisors
///
/// ```
/// use project_euler_practice::prime::get_num_divisors as f;
/// assert_eq!(f(2), 2);
/// assert_eq!(f(6), 4);
/// assert_eq!(f(9), 3);
/// ```
pub fn get_num_divisors(mut n: u64) -> u64 {
    let mut d = 2;
    let mut result = 1;
    while n > 1 {
        let mut count = 1;
        while n % d == 0 {
            n /= d;
            count += 1;
        }
        d += 1;
        result *= count;
    }
    return result;
}
