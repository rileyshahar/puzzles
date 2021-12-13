//! What is the sum of the digits of the number 2**1000?
use num::BigUint;

fn solve_for(exponent: u32) -> u16 {
    BigUint::from(2u8)
        .pow(exponent)
        .to_radix_be(10)
        .iter()
        .map(|&n| u16::from(n))
        .sum()
}

super::example!(15 => 26);
super::problem!(u16: 1000 => 1366);
