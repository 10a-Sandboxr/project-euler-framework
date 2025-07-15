use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};
use std::collections::HashMap;

pub fn digit_sum(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

pub fn digit_sum_big(n: &BigUint) -> u64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

pub fn digital_root(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        1 + (n - 1) % 9
    }
}

pub fn reverse_number(n: u64) -> u64 {
    let mut result = 0;
    let mut num = n;
    
    while num > 0 {
        result = result * 10 + num % 10;
        num /= 10;
    }
    
    result
}

pub fn is_palindrome(n: u64) -> bool {
    n == reverse_number(n)
}

pub fn is_palindrome_string(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    
    true
}

pub fn digit_count(n: u64) -> usize {
    if n == 0 {
        1
    } else {
        n.to_string().len()
    }
}

pub fn digits_of_number(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

pub fn number_from_digits(digits: &[u8]) -> u64 {
    digits.iter()
        .fold(0, |acc, &digit| acc * 10 + digit as u64)
}

pub fn is_pandigital(n: u64, length: usize) -> bool {
    let digits = digits_of_number(n);
    
    if digits.len() != length {
        return false;
    }
    
    let mut digit_set = vec![false; length + 1];
    
    for digit in digits {
        if digit == 0 || digit > length as u8 || digit_set[digit as usize] {
            return false;
        }
        digit_set[digit as usize] = true;
    }
    
    digit_set[1..=length].iter().all(|&x| x)
}

pub fn factorial_digit_sum(n: u64) -> u64 {
    let mut factorial = BigUint::one();
    for i in 1..=n {
        factorial *= BigUint::from(i);
    }
    digit_sum_big(&factorial)
}

pub fn power_digit_sum(base: u64, exp: u64) -> u64 {
    let result = BigUint::from(base).pow(exp as u32);
    digit_sum_big(&result)
}

pub fn is_abundant(n: u64) -> bool {
    use crate::number_theory::sum_of_proper_divisors;
    sum_of_proper_divisors(n) > n
}

pub fn is_deficient(n: u64) -> bool {
    use crate::number_theory::sum_of_proper_divisors;
    sum_of_proper_divisors(n) < n
}

pub fn is_perfect(n: u64) -> bool {
    use crate::number_theory::sum_of_proper_divisors;
    sum_of_proper_divisors(n) == n
}

pub fn amicable_pair(n: u64) -> Option<u64> {
    use crate::number_theory::sum_of_proper_divisors;
    let sum_n = sum_of_proper_divisors(n);
    let sum_sum_n = sum_of_proper_divisors(sum_n);
    
    if sum_sum_n == n && sum_n != n {
        Some(sum_n)
    } else {
        None
    }
}

pub fn happy_number(mut n: u64) -> bool {
    let mut seen = std::collections::HashSet::new();
    
    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = digits_of_number(n)
            .iter()
            .map(|&d| (d as u64) * (d as u64))
            .sum();
    }
    
    n == 1
}

pub fn sum_of_squares_of_digits(n: u64) -> u64 {
    digits_of_number(n)
        .iter()
        .map(|&d| (d as u64) * (d as u64))
        .sum()
}

pub fn harshad_number(n: u64) -> bool {
    let sum = digit_sum(n);
    sum > 0 && n % sum == 0
}

pub fn kaprekar_number(n: u64) -> bool {
    let square = n * n;
    let square_str = square.to_string();
    let len = square_str.len();
    
    for i in 1..len {
        let left = &square_str[..i];
        let right = &square_str[i..];
        
        if !right.is_empty() && !left.is_empty() {
            let left_num: u64 = left.parse().unwrap_or(0);
            let right_num: u64 = right.parse().unwrap_or(0);
            
            if right_num > 0 && left_num + right_num == n {
                return true;
            }
        }
    }
    
    false
}

pub fn armstrong_number(n: u64) -> bool {
    let digits = digits_of_number(n);
    let power = digits.len() as u32;
    let sum: u64 = digits.iter()
        .map(|&d| (d as u64).pow(power))
        .sum();
    sum == n
}

pub fn sum_of_divisors_sigma(n: u64) -> u64 {
    use crate::number_theory::divisors;
    divisors(n).iter().sum()
}

pub fn multiplicative_order(a: u64, n: u64) -> Option<u64> {
    use crate::number_theory::gcd;
    
    if gcd(a, n) != 1 {
        return None;
    }
    
    let mut power = 1;
    let mut current = a % n;
    
    while current != 1 {
        current = (current * a) % n;
        power += 1;
        
        if power > n {
            return None;
        }
    }
    
    Some(power)
}

pub fn continued_fraction_sqrt(n: u64) -> Vec<u64> {
    if is_perfect_square(n) {
        return vec![(n as f64).sqrt() as u64];
    }
    
    let mut result = Vec::new();
    let a0 = (n as f64).sqrt() as u64;
    result.push(a0);
    
    let mut m = 0;
    let mut d = 1;
    let mut a = a0;
    
    let mut seen = HashMap::new();
    
    loop {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        
        if let Some(&start) = seen.get(&(m, d)) {
            break;
        }
        
        seen.insert((m, d), result.len());
        result.push(a);
    }
    
    result
}

pub fn is_perfect_square(n: u64) -> bool {
    let sqrt_n = (n as f64).sqrt() as u64;
    sqrt_n * sqrt_n == n
}

pub fn totient_sum(n: u64) -> u64 {
    use crate::number_theory::euler_totient;
    (1..=n).map(euler_totient).sum()
}

pub fn binary_representation(n: u64) -> String {
    format!("{:b}", n)
}

pub fn count_bits(n: u64) -> u32 {
    n.count_ones()
}

pub fn gray_code(n: u64) -> u64 {
    n ^ (n >> 1)
}

pub fn reverse_gray_code(g: u64) -> u64 {
    let mut n = g;
    let mut mask = g >> 1;
    
    while mask != 0 {
        n ^= mask;
        mask >>= 1;
    }
    
    n
}

pub fn hamming_weight(n: u64) -> u32 {
    n.count_ones()
}

pub fn base_conversion(n: u64, base: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    
    let mut result = String::new();
    let mut num = n;
    
    while num > 0 {
        let digit = num % base;
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'A' + (digit - 10) as u8) as char
        };
        result.push(char);
        num /= base;
    }
    
    result.chars().rev().collect()
}

pub fn from_base(s: &str, base: u64) -> u64 {
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_digit(10).unwrap() as u64
            } else {
                (c.to_ascii_uppercase() as u64) - (b'A' as u64) + 10
            }
        })
        .fold(0, |acc, digit| acc * base + digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum() {
        assert_eq!(digit_sum(123), 6);
        assert_eq!(digit_sum(9875), 29);
        assert_eq!(digit_sum(0), 0);
    }

    #[test]
    fn test_digital_root() {
        assert_eq!(digital_root(123), 6);
        assert_eq!(digital_root(9875), 2);
        assert_eq!(digital_root(0), 0);
    }

    #[test]
    fn test_reverse_number() {
        assert_eq!(reverse_number(123), 321);
        assert_eq!(reverse_number(1000), 1);
        assert_eq!(reverse_number(0), 0);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(1221));
        assert!(!is_palindrome(123));
        assert!(is_palindrome(0));
    }

    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital(123, 3));
        assert!(is_pandigital(321, 3));
        assert!(!is_pandigital(112, 3));
        assert!(!is_pandigital(1234, 3));
    }

    #[test]
    fn test_happy_number() {
        assert!(happy_number(1));
        assert!(happy_number(7));
        assert!(happy_number(10));
        assert!(!happy_number(2));
    }

    #[test]
    fn test_armstrong_number() {
        assert!(armstrong_number(153));
        assert!(armstrong_number(9474));
        assert!(!armstrong_number(123));
    }

    #[test]
    fn test_base_conversion() {
        assert_eq!(base_conversion(10, 2), "1010");
        assert_eq!(base_conversion(255, 16), "FF");
        assert_eq!(base_conversion(0, 10), "0");
    }

    #[test]
    fn test_from_base() {
        assert_eq!(from_base("1010", 2), 10);
        assert_eq!(from_base("FF", 16), 255);
        assert_eq!(from_base("0", 10), 0);
    }
}