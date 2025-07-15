pub mod number_theory;
pub mod combinatorics;
pub mod sequences;
pub mod utilities;

pub use number_theory::*;
pub use combinatorics::*;
pub use sequences::*;
pub use utilities::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert!(is_prime(17));
        assert!(!is_prime(15));
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(fibonacci(10), 55);
    }
}