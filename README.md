# Project Euler Framework

A comprehensive mathematical framework written in Rust for solving Project Euler problems. This library provides efficient implementations of common mathematical functions, algorithms, and utilities that frequently appear in Project Euler challenges.

## Features

- **Number Theory**: Prime generation, factorization, GCD/LCM, modular arithmetic, and more
- **Combinatorics**: Permutations, combinations, factorials, and special counting functions
- **Sequences**: Fibonacci, triangular, pentagonal, and other mathematical sequences
- **Utilities**: Digital operations, palindromes, base conversions, and number properties

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
project-euler-framework = "0.1.0"
```

## Quick Start

```rust
use project_euler_framework::*;

fn main() {
    // Check if a number is prime
    println!("Is 17 prime? {}", is_prime(17));
    
    // Generate the first 10 Fibonacci numbers
    let fib_sequence = (0..10).map(fibonacci).collect::<Vec<_>>();
    println!("First 10 Fibonacci numbers: {:?}", fib_sequence);
    
    // Calculate combinations
    println!("C(10, 3) = {}", combinations(10, 3));
    
    // Find prime factors
    println!("Prime factors of 60: {:?}", prime_factors(60));
}
```

## Module Overview

### Number Theory (`number_theory`)

Functions for working with prime numbers, factorization, and modular arithmetic.

#### Prime Numbers
```rust
// Check if a number is prime
assert!(is_prime(17));
assert!(!is_prime(15));

// Generate primes up to a limit using Sieve of Eratosthenes
let primes = primes_up_to(100);
println!("Primes up to 100: {:?}", primes);

// Get the nth prime
let tenth_prime = nth_prime(10);
println!("10th prime: {}", tenth_prime);
```

#### Factorization
```rust
// Get prime factors
let factors = prime_factors(60);
println!("Prime factors of 60: {:?}", factors); // [2, 2, 3, 5]

// Get prime factorization as a map
let factorization = prime_factorization(60);
println!("60 = {:?}", factorization); // {2: 2, 3: 1, 5: 1}

// Find all divisors
let divisors = divisors(12);
println!("Divisors of 12: {:?}", divisors); // [1, 2, 3, 4, 6, 12]
```

#### GCD and LCM
```rust
// Greatest Common Divisor
assert_eq!(gcd(48, 18), 6);

// Least Common Multiple
assert_eq!(lcm(4, 6), 12);

// Extended Euclidean Algorithm
let (g, x, y) = extended_gcd(48, 18);
println!("gcd(48, 18) = {}, 48*{} + 18*{} = {}", g, x, y, g);
```

#### Modular Arithmetic
```rust
// Modular exponentiation
let result = modular_exponentiation(2, 10, 1000);
println!("2^10 mod 1000 = {}", result);

// Modular inverse
if let Some(inv) = modular_inverse(3, 7) {
    println!("3^-1 mod 7 = {}", inv);
}
```

### Combinatorics (`combinatorics`)

Functions for counting, permutations, and combinations.

#### Basic Counting
```rust
// Factorial
assert_eq!(factorial(5), 120);

// For large factorials, use BigUint
let big_fact = factorial_big(100);
println!("100! = {}", big_fact);

// Permutations P(n,r) = n!/(n-r)!
assert_eq!(permutations(5, 2), 20);

// Combinations C(n,r) = n!/(r!(n-r)!)
assert_eq!(combinations(5, 2), 10);
```

#### Advanced Combinatorics
```rust
// Catalan numbers
assert_eq!(catalan_number(4), 14);

// Bell numbers (number of partitions of a set)
let bell_5 = bell_number(5);
println!("Bell(5) = {}", bell_5);

// Stirling numbers of the second kind
let stirling = stirling_second_kind(5, 3);
println!("S(5,3) = {}", stirling);

// Partition function
let partitions = partition_function(5);
println!("Number of partitions of 5: {}", partitions);
```

#### Generating Sequences
```rust
// Generate all permutations
let items = vec![1, 2, 3];
let perms = generate_permutations(&items);
println!("All permutations of [1,2,3]: {:?}", perms);

// Generate all combinations
let combos = generate_combinations(&items, 2);
println!("All 2-combinations of [1,2,3]: {:?}", combos);
```

### Sequences (`sequences`)

Functions for generating and working with mathematical sequences.

#### Fibonacci Sequence
```rust
// nth Fibonacci number
assert_eq!(fibonacci(10), 55);

// For large Fibonacci numbers
let big_fib = fibonacci_big(100);
println!("F(100) = {}", big_fib);

// Generate Fibonacci sequence up to a limit
let fib_seq = fibonacci_sequence(100);
println!("Fibonacci numbers ≤ 100: {:?}", fib_seq);
```

#### Polygonal Numbers
```rust
// Triangular numbers
assert_eq!(triangular_number(10), 55);
assert!(is_triangular(55));

// Pentagonal numbers
assert_eq!(pentagonal_number(3), 12);
assert!(is_pentagonal(12));

// Hexagonal numbers
assert_eq!(hexagonal_number(3), 15);

// General polygonal numbers
let heptagonal_5 = polygonal_number(5, 7);
println!("5th heptagonal number: {}", heptagonal_5);
```

#### Other Sequences
```rust
// Lucas numbers
assert_eq!(lucas_number(5), 11);

// Collatz sequence
let collatz = collatz_sequence(7);
println!("Collatz sequence for 7: {:?}", collatz);

// Collatz sequence length
let length = collatz_length(7);
println!("Collatz length for 7: {}", length);

// Look-and-say sequence
let look_say = look_and_say_sequence("1", 5);
println!("Look-and-say starting with '1': {:?}", look_say);
```

### Utilities (`utilities`)

Utility functions for digit manipulation and number properties.

#### Digit Operations
```rust
// Sum of digits
assert_eq!(digit_sum(123), 6);

// Digital root
assert_eq!(digital_root(123), 6);

// Reverse a number
assert_eq!(reverse_number(123), 321);

// Check if palindrome
assert!(is_palindrome(121));

// Get individual digits
let digits = digits_of_number(123);
println!("Digits of 123: {:?}", digits); // [1, 2, 3]

// Build number from digits
let number = number_from_digits(&[1, 2, 3]);
assert_eq!(number, 123);
```

#### Number Properties
```rust
// Check if pandigital
assert!(is_pandigital(123, 3));

// Check if perfect, abundant, or deficient
assert!(is_perfect(6));
assert!(is_abundant(12));
assert!(is_deficient(8));

// Happy numbers
assert!(happy_number(7));

// Armstrong numbers
assert!(armstrong_number(153));

// Harshad numbers
assert!(harshad_number(12));
```

#### Base Conversions
```rust
// Convert to different bases
let binary = base_conversion(10, 2);
println!("10 in binary: {}", binary); // "1010"

let hex = base_conversion(255, 16);
println!("255 in hex: {}", hex); // "FF"

// Convert from different bases
let decimal = from_base("1010", 2);
assert_eq!(decimal, 10);

let from_hex = from_base("FF", 16);
assert_eq!(from_hex, 255);
```

## Example Solutions

Here are some examples of how to use this framework to solve Project Euler problems:

### Problem 1: Sum of multiples of 3 or 5 below 1000
```rust
use project_euler_framework::*;

fn problem_1() -> u64 {
    (1..1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}
```

### Problem 2: Sum of even Fibonacci numbers below 4 million
```rust
use project_euler_framework::*;

fn problem_2() -> u64 {
    fibonacci_sequence(4_000_000)
        .into_iter()
        .filter(|&n| n % 2 == 0)
        .sum()
}
```

### Problem 3: Largest prime factor of 600851475143
```rust
use project_euler_framework::*;

fn problem_3() -> u64 {
    *prime_factors(600851475143).last().unwrap()
}
```

### Problem 4: Largest palindrome product of two 3-digit numbers
```rust
use project_euler_framework::*;

fn problem_4() -> u64 {
    let mut largest = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome(product) && product > largest {
                largest = product;
            }
        }
    }
    largest
}
```

### Problem 5: Smallest multiple of 1 to 20
```rust
use project_euler_framework::*;

fn problem_5() -> u64 {
    (1..=20).fold(1, |acc, n| lcm(acc, n))
}
```

## Performance Considerations

- Functions use `u64` for most calculations to balance performance and range
- For very large numbers, use the `*_big` variants that return `BigUint`
- The sieve of Eratosthenes is the most efficient way to generate many primes
- Memoization is used internally where beneficial

## Dependencies

- `num-bigint`: For arbitrary precision arithmetic
- `num-traits`: For numeric trait abstractions
- `rayon`: For parallel processing (future feature)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for:

- Additional mathematical functions
- Performance improvements
- Bug fixes
- Documentation improvements
- More example solutions

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the mathematical challenges of Project Euler
- Built with the Rust programming language for performance and safety
- Mathematical algorithms sourced from various computational mathematics references