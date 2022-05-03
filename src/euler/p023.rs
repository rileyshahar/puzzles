//! Find the sum of all the positive integers which cannot be written as the sum of two abundant
//! numbers.
use std::cmp::Ordering;

use crate::utils::seqs::Abundant;

// Does the ordered list contain two elements which sum to the target?
fn contains_two_element_sum(list: &[u32], target: u32) -> bool {
    let mut begin = 0;
    let mut end = list.len() - 1;

    // successively update the upper and lower bounds on the elements that can be involved in the
    // sum, until we hit the correct sum
    while begin < end {
        let curr = list[begin] + list[end];
        match curr.cmp(&target) {
            Ordering::Less => begin += 1,
            Ordering::Equal => return true,
            Ordering::Greater => end -= 1,
        }
    }

    // special case for begin = end, in case we have end = 0, when we can't subtract because of
    // underflow
    list[begin] + list[end] == target
}

fn solve_for(max: u32) -> u32 {
    let abundants: Vec<_> = Abundant::<u32>::new().take_while(|&n| n < max).collect();

    (1..max)
        .filter(|&n| !contains_two_element_sum(&abundants, n))
        .sum()
}

super::example!(31 => 411); // every number except 24 and 30 can't be written as the sum of two
                            // abundant numbers
super::problem!(u32: 28124 => 4_179_871); // 28123 is a known theoretical limit, per the problem
