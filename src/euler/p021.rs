//! Evaluate the sum of all the amicable numbers under 10000.
use std::collections::HashMap;

use crate::utils::primes::{PrimeFactor, PrimeFactorization};

type Cache = HashMap<u32, u32>;

/// Is n the smaller number in an amicable pair?
///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
/// into n). If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a
/// and b are called amicable numbers.
fn is_amicable(n: u32, cache: &mut Cache) -> bool {
    // TODO: make a well-behaved generic cache type
    let potential_partner = *cache.entry(n).or_insert_with(|| proper_divisor_sum(n));

    if potential_partner == n {
        // n is perfect, not amicable
        return false;
    }

    return cache
        .entry(potential_partner)
        .or_insert_with(|| proper_divisor_sum(potential_partner))
        == &n;
}

/// What is the sum of proper divisors of n?
fn proper_divisor_sum(n: u32) -> u32 {
    if n == 0 {
        // our trick doesn't work for 0
        return 0;
    }

    // each divisor is a product of prime factors with some exponents less than the exponent of the
    // factor in the prime decomposition of n. we exploit this plus distributivity to write the sum
    // of divisors as a product of geometric series.
    PrimeFactorization::of(n)
        // this uses the formula for a geometric series
        .map(|PrimeFactor { factor, exponent }| ((factor).pow(exponent + 1) - 1) / (factor - 1))
        .product::<u32>()
        - n // we only want proper divisors
}

fn solve_for(max: u32) -> u32 {
    let mut cache = Cache::new();

    (1..max).filter(|&n| is_amicable(n, &mut cache)).sum()
}

super::example!(300 => 504); // just 220 and 284
super::problem!(u32: 10000);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_amicable_test() {
        // make a new cache on each one just for independence
        assert!(is_amicable(220, &mut Cache::new()));
        assert!(is_amicable(284, &mut Cache::new()));
        assert!(!is_amicable(28, &mut Cache::new())); // perfect number
        assert!(!is_amicable(127, &mut Cache::new())); // just a random number
        assert!(!is_amicable(1, &mut Cache::new()));
    }

    #[test]
    fn proper_divisor_sum_test() {
        assert!(proper_divisor_sum(220) == 284);
    }
}
