use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};
use std::collections::HashMap;

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 {
        is_prime[1] = false;
    }
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

pub fn primes_up_to(limit: usize) -> Vec<u64> {
    let is_prime = sieve_of_eratosthenes(limit);
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i as u64) } else { None })
        .collect()
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    
    if n > 1 {
        factors.push(n);
    }
    
    factors
}

pub fn prime_factorization(n: u64) -> HashMap<u64, u32> {
    let factors = prime_factors(n);
    let mut factorization = HashMap::new();
    
    for factor in factors {
        *factorization.entry(factor).or_insert(0) += 1;
    }
    
    factorization
}

pub fn divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u64;
    
    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    
    divisors.sort();
    divisors
}

pub fn sum_of_divisors(n: u64) -> u64 {
    divisors(n).iter().sum()
}

pub fn proper_divisors(n: u64) -> Vec<u64> {
    let mut divs = divisors(n);
    if !divs.is_empty() {
        divs.pop();
    }
    divs
}

pub fn sum_of_proper_divisors(n: u64) -> u64 {
    proper_divisors(n).iter().sum()
}

pub fn euler_totient(n: u64) -> u64 {
    let factors = prime_factorization(n);
    let mut result = n;
    
    for &prime in factors.keys() {
        result = result - result / prime;
    }
    
    result
}

pub fn modular_exponentiation(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    
    result
}

pub fn modular_inverse(a: u64, m: u64) -> Option<u64> {
    let (g, x, _) = extended_gcd(a as i64, m as i64);
    if g != 1 {
        None
    } else {
        Some(((x % m as i64 + m as i64) % m as i64) as u64)
    }
}

pub fn is_perfect_square(n: u64) -> bool {
    let sqrt_n = (n as f64).sqrt() as u64;
    sqrt_n * sqrt_n == n
}

pub fn nth_prime(n: usize) -> u64 {
    if n == 1 {
        return 2;
    }
    
    let mut count = 1;
    let mut candidate = 3;
    
    while count < n {
        if is_prime(candidate) {
            count += 1;
        }
        if count < n {
            candidate += 2;
        }
    }
    
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(17));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(17), vec![17]);
        assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
    }

    #[test]
    fn test_divisors() {
        let mut divs = divisors(12);
        divs.sort();
        assert_eq!(divs, vec![1, 2, 3, 4, 6, 12]);
    }
}