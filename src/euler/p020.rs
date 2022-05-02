//! What is the sum of the digits of the number 100!?
use num::BigUint;

fn solve_for(n: u32) -> u64 {
    (1..=n)
        .fold(BigUint::from(1u8), |curr, i| curr * i)
        .to_radix_be(10)
        .iter()
        .map(|&n| u64::from(n))
        .sum()
}

super::example!(10 => 27);
super::problem!(u64: 100 => 648);
