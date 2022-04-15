//! If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//!
//! This one is a little silly to be honest.

fn count_letters(n: u32) -> u32 {
    #[allow(clippy::match_same_arms)]
    match n {
        1 => 3,
        2 => 3,
        3 => 5,
        4 => 4,
        5 => 4,
        6 => 3,
        7 => 5,
        8 => 5,
        9 => 4,
        10 => 3,
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 | 17 | 19 => count_letters(n % 10) + 4, // 4 is for "teen"
        18 => 8,
        20 => 6,
        30 => 6,
        40 => 5,
        50 => 5,
        60 | 70 | 90 => count_letters(n / 10) + 2, // 2 is for "ty"
        80 => 6,
        21..=99 => count_letters(n / 10 * 10) + count_letters(n % 10),
        100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => count_letters(n / 100) + 7, // 7 is for "hundred"
        101..=999 => count_letters(n / 100 * 100) + 3 + count_letters(n % 100), // 3 is for "and"
        1000 => 11,
        _ => panic!("only supports numbers from 1 to 1000: got {}", n),
    }
}

fn solve_for(bound: u32) -> u32 {
    (1..=bound).map(count_letters).sum()
}

super::example!(5 => 19);
super::problem!(u32: 1000 => 21124);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_letters_works() {
        assert_eq!(count_letters(17), 9);
        assert_eq!(count_letters(32), 9);
        assert_eq!(count_letters(115), 20);
        assert_eq!(count_letters(204), 17);
        assert_eq!(count_letters(324), 25);
        assert_eq!(count_letters(342), 23);
        assert_eq!(count_letters(412), 20);
        assert_eq!(count_letters(482), 23);
        assert_eq!(count_letters(700), 12);
        assert_eq!(count_letters(732), 24);
        assert_eq!(count_letters(999), 24);
    }
}
