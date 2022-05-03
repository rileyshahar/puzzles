//! Evaluate the sum of all the amicable numbers under 10000.
use std::collections::HashMap;

use crate::utils::primes::unique_proper_divisor_sum;

type Cache = HashMap<u32, u32>;

/// Is n the smaller number in an amicable pair?
///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
/// into n). If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a
/// and b are called amicable numbers.
fn is_amicable(n: u32, cache: &mut Cache) -> bool {
    // TODO: make a well-behaved generic cache type
    let potential_partner = *cache
        .entry(n)
        .or_insert_with(|| unique_proper_divisor_sum(n));

    if potential_partner == n {
        // n is perfect, not amicable
        return false;
    }

    return cache
        .entry(potential_partner)
        .or_insert_with(|| unique_proper_divisor_sum(potential_partner))
        == &n;
}

fn solve_for(max: u32) -> u32 {
    let mut cache = Cache::new();

    (1..max).filter(|&n| is_amicable(n, &mut cache)).sum()
}

super::example!(300 => 504); // just 220 and 284
super::problem!(u32: 10000 => 31626);

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
}
