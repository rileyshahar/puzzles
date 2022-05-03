//! Find the sum of all the positive integers which cannot be written as the sum of two abundant
//! numbers.
use crate::utils::seqs::Abundant;

fn solve_for(max: u32) -> u32 {
    let abundants: Vec<_> = Abundant::<u32>::new().take_while(|&n| n < max).collect();

    let mut is_sum_of_two_abundants = vec![false; max as usize];

    for (i, &a) in abundants.iter().enumerate() {
        for b in abundants[i..].iter() {
            let sum = a + b;
            if sum >= max {
                // clearly no more sums for this a will be relevant to us
                break;
            }
            is_sum_of_two_abundants[sum as usize] = true;
        }
    }

    (1..max)
        .filter(|&i| !is_sum_of_two_abundants[i as usize])
        .sum()
}

super::example!(31 => 411); // every number except 24 and 30 can't be written as the sum of two
                            // abundant numbers
super::problem!(u32: 28124 => 4_179_871); // 28123 is a known theoretical limit, per the problem
