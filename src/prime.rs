use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PrimeTester {
    cache: Vec<bool>,
}

impl PrimeTester {
    // adds in any missing falses in the cahce. Won't restore improper false vals
    fn seive_cache(&mut self) -> () {
        self.cache[0] = false;
        self.cache[1] = false;
        for i in 2..self.cache.len() {
            if self.cache[i] {
                let mut multiples = i + i;
                while multiples < self.cache.len() {
                    self.cache[multiples] = false;
                    multiples += i;
                }
            }
        }
    }

    // doubles the size of the cached seive
    fn grow_cache(&mut self) -> () {
        self.cache.extend(vec![true; self.cache.len()]);
        self.seive_cache();
    }

    fn grow_cache_to(&mut self, n: usize) -> () {
        while self.cache.len() <= n {
            self.grow_cache();
        }
    }

    pub fn new() -> PrimeTester {
        PrimeTester {
            cache: vec![false, false, true, true, false, true],
        }
    }

    pub fn init(mut n: usize) -> PrimeTester {
        n = std::cmp::max(n, 5);
        let mut pt = PrimeTester {
            cache: vec![true; n + 1],
        };
        pt.seive_cache();
        return pt;
    }

    /// returns if the given number is prime
    ///
    /// grows the cache up to n, so memory intensive for large n
    ///
    /// ```
    /// use project_euler_practice::prime::PrimeTester;
    /// let mut pt = PrimeTester::new();
    /// assert_eq!(pt.is_prime(1), false);
    /// assert_eq!(pt.is_prime(2), true);
    /// assert_eq!(pt.is_prime(17), true);
    /// assert_eq!(pt.is_prime(25), false);
    /// ```
    pub fn is_prime(&mut self, n: u64) -> bool {
        self.grow_cache_to(n as usize);
        return self.cache[n as usize];
    }

    /// returns all prime numbers up to max, after growing the cache to max
    ///
    /// ```
    /// use project_euler_practice::prime::PrimeTester;
    /// let mut pt = PrimeTester::new();
    /// let primes = pt.get_primes(100);
    /// for i in 1..100 {
    ///     assert_eq!(pt.is_prime(i), primes.contains(&i));
    /// }
    /// ```
    pub fn get_primes(&mut self, max: usize) -> HashSet<u64> {
        self.grow_cache_to(max as usize);
        return self
            .cache
            .iter()
            .enumerate()
            .filter(|(i, &b)| b && (*i <= max as usize))
            .map(|(i, _)| i as u64)
            .collect();
    }

    /// returns all prime numbers up to max, after growing the cache to max
    ///
    /// ```
    /// use project_euler_practice::prime::PrimeTester;
    /// let mut pt = PrimeTester::new();
    /// let primes = pt.get_primes_ord(11);
    /// assert_eq!(primes, vec!(2, 3, 5, 7, 11))
    /// ```
    pub fn get_primes_ord(&mut self, max: usize) -> Vec<u64> {
        let mut result = Vec::with_capacity(max / 2);
        self.grow_cache_to(max);
        for i in 2..=max {
            if self.cache[i] {
                result.push(i as u64);
            }
        }
        return result;
    }
}

/// returns if the given number is prime
///
/// This version doesn't cache primes, so is inefficient over multiple calls
/// it is however more effective with very large n since it doesn't cache
///
/// ```
/// use project_euler_practice::prime::is_prime;
/// assert_eq!(is_prime(1), false);
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(17), true);
/// assert_eq!(is_prime(25), false);
/// assert_eq!(is_prime(12512345122), false);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n == 1 || n % 2 == 0 || n % 3 == 0 {
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
/// assert_eq!(divs.len(), 23);
/// for i in 1..630 {
///     if 630 % i == 0 {
///         assert!(divs.contains(&i));
///     }
/// }
/// ```
pub fn get_divisors(n: u64) -> HashSet<u64> {
    let mut divisors = HashSet::with_capacity(get_num_divisors(n) as usize);
    divisors.insert(1);
    let max = (n as f64).sqrt() as u64;
    for i in 2..=max {
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

/// gets the number of distinct prime divisors of an integer
///
/// ```
/// use project_euler_practice::prime::count_distinct_prime_divisors as f;
/// assert_eq!(f(14), 2);
/// assert_eq!(f(15), 2);
/// assert_eq!(f(16), 1);
/// assert_eq!(f(644), 3);
/// assert_eq!(f(645), 3);
/// assert_eq!(f(646), 3);
/// ```
pub fn count_distinct_prime_divisors(mut n: u64) -> u64 {
    let mut d = 2;
    let mut result = 0;
    while n > 1 {
        if n % d == 0 {
            result += 1;
            n /= d;
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    return result;
}

/// Determine if the two numbers are relatively prime.
/// The first argument must be the smaller of the two numbers.
///
/// ```
/// use project_euler_practice::prime::is_relatively_prime as f;
/// assert!(f(1,4));
/// assert!(f(1,5012));
/// assert!(f(5,6));
/// assert!(f(3,8));
/// assert!(!f(6,12));
/// assert!(!f(6,9));
/// assert!(!f(2506,5012));
/// ```
pub fn is_relatively_prime(small: u64, large: u64) -> bool {
    if small % 2 == 0 && large % 2 == 0 {
        return false;
    }
    for i in (3..=small).step_by(2) {
        if small % i == 0 && large % i == 0 {
            return false;
        }
    }
    return true;
}
