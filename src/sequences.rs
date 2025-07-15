use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};
use std::collections::HashMap;

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

pub fn fibonacci_big(n: u64) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        _ => {
            let mut a = BigUint::zero();
            let mut b = BigUint::one();
            for _ in 2..=n {
                let temp = &a + &b;
                a = std::mem::replace(&mut b, temp);
            }
            b
        }
    }
}

pub fn fibonacci_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = vec![0, 1];
    let mut a = 0;
    let mut b = 1;
    
    loop {
        let next = a + b;
        if next > limit {
            break;
        }
        sequence.push(next);
        a = b;
        b = next;
    }
    
    sequence
}

pub fn triangular_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn is_triangular(num: u64) -> bool {
    let discriminant = 1 + 8 * num;
    let sqrt_discriminant = (discriminant as f64).sqrt() as u64;
    sqrt_discriminant * sqrt_discriminant == discriminant && (sqrt_discriminant - 1) % 2 == 0
}

pub fn triangular_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut n = 1;
    
    loop {
        let tri = triangular_number(n);
        if tri > limit {
            break;
        }
        sequence.push(tri);
        n += 1;
    }
    
    sequence
}

pub fn pentagonal_number(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

pub fn is_pentagonal(num: u64) -> bool {
    let discriminant = 1 + 24 * num;
    let sqrt_discriminant = (discriminant as f64).sqrt() as u64;
    sqrt_discriminant * sqrt_discriminant == discriminant && (sqrt_discriminant + 1) % 6 == 0
}

pub fn pentagonal_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut n = 1;
    
    loop {
        let pent = pentagonal_number(n);
        if pent > limit {
            break;
        }
        sequence.push(pent);
        n += 1;
    }
    
    sequence
}

pub fn hexagonal_number(n: u64) -> u64 {
    n * (2 * n - 1)
}

pub fn is_hexagonal(num: u64) -> bool {
    let discriminant = 1 + 8 * num;
    let sqrt_discriminant = (discriminant as f64).sqrt() as u64;
    sqrt_discriminant * sqrt_discriminant == discriminant && (sqrt_discriminant + 1) % 4 == 0
}

pub fn hexagonal_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut n = 1;
    
    loop {
        let hex = hexagonal_number(n);
        if hex > limit {
            break;
        }
        sequence.push(hex);
        n += 1;
    }
    
    sequence
}

pub fn heptagonal_number(n: u64) -> u64 {
    n * (5 * n - 3) / 2
}

pub fn octagonal_number(n: u64) -> u64 {
    n * (3 * n - 2)
}

pub fn polygonal_number(n: u64, sides: u64) -> u64 {
    match sides {
        3 => triangular_number(n),
        4 => n * n,
        5 => pentagonal_number(n),
        6 => hexagonal_number(n),
        7 => heptagonal_number(n),
        8 => octagonal_number(n),
        _ => n * ((sides - 2) * n - (sides - 4)) / 2,
    }
}

pub fn square_number(n: u64) -> u64 {
    n * n
}

pub fn is_perfect_square(num: u64) -> bool {
    let sqrt_num = (num as f64).sqrt() as u64;
    sqrt_num * sqrt_num == num
}

pub fn square_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut n = 1;
    
    loop {
        let square = square_number(n);
        if square > limit {
            break;
        }
        sequence.push(square);
        n += 1;
    }
    
    sequence
}

pub fn cube_number(n: u64) -> u64 {
    n * n * n
}

pub fn is_perfect_cube(num: u64) -> bool {
    let cbrt_num = (num as f64).cbrt() as u64;
    cbrt_num * cbrt_num * cbrt_num == num
}

pub fn cube_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut n = 1;
    
    loop {
        let cube = cube_number(n);
        if cube > limit {
            break;
        }
        sequence.push(cube);
        n += 1;
    }
    
    sequence
}

pub fn lucas_number(n: u64) -> u64 {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            let mut a = 2;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

pub fn lucas_sequence(limit: u64) -> Vec<u64> {
    let mut sequence = vec![2, 1];
    let mut a = 2;
    let mut b = 1;
    
    loop {
        let next = a + b;
        if next > limit {
            break;
        }
        sequence.push(next);
        a = b;
        b = next;
    }
    
    sequence
}

pub fn tribonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut c = 1;
            for _ in 3..=n {
                let temp = a + b + c;
                a = b;
                b = c;
                c = temp;
            }
            c
        }
    }
}

pub fn collatz_sequence(mut n: u64) -> Vec<u64> {
    let mut sequence = vec![n];
    
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        sequence.push(n);
    }
    
    sequence
}

pub fn collatz_length(mut n: u64) -> u64 {
    let mut length = 1;
    
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    
    length
}

pub fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    if chars.is_empty() {
        return result;
    }
    
    let mut current_char = chars[0];
    let mut count = 1;
    
    for i in 1..chars.len() {
        if chars[i] == current_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current_char);
            current_char = chars[i];
            count = 1;
        }
    }
    
    result.push_str(&count.to_string());
    result.push(current_char);
    
    result
}

pub fn look_and_say_sequence(start: &str, iterations: usize) -> Vec<String> {
    let mut sequence = vec![start.to_string()];
    let mut current = start.to_string();
    
    for _ in 0..iterations {
        current = look_and_say(&current);
        sequence.push(current.clone());
    }
    
    sequence
}

pub fn padovan_number(n: u64) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        _ => {
            let mut a = 1;
            let mut b = 1;
            let mut c = 1;
            for _ in 3..=n {
                let temp = a + b;
                a = b;
                b = c;
                c = temp;
            }
            c
        }
    }
}

pub fn pronic_number(n: u64) -> u64 {
    n * (n + 1)
}

pub fn is_pronic(num: u64) -> bool {
    let sqrt_num = (num as f64).sqrt() as u64;
    sqrt_num * (sqrt_num + 1) == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);
    }

    #[test]
    fn test_triangular_number() {
        assert_eq!(triangular_number(1), 1);
        assert_eq!(triangular_number(3), 6);
        assert_eq!(triangular_number(10), 55);
    }

    #[test]
    fn test_is_triangular() {
        assert!(is_triangular(1));
        assert!(is_triangular(3));
        assert!(is_triangular(6));
        assert!(is_triangular(10));
        assert!(!is_triangular(2));
        assert!(!is_triangular(4));
    }

    #[test]
    fn test_pentagonal_number() {
        assert_eq!(pentagonal_number(1), 1);
        assert_eq!(pentagonal_number(2), 5);
        assert_eq!(pentagonal_number(3), 12);
    }

    #[test]
    fn test_collatz_length() {
        assert_eq!(collatz_length(1), 1);
        assert_eq!(collatz_length(2), 2);
        assert_eq!(collatz_length(3), 8);
        assert_eq!(collatz_length(4), 3);
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
    }

    #[test]
    fn test_is_perfect_square() {
        assert!(is_perfect_square(1));
        assert!(is_perfect_square(4));
        assert!(is_perfect_square(9));
        assert!(is_perfect_square(16));
        assert!(!is_perfect_square(2));
        assert!(!is_perfect_square(3));
    }
}