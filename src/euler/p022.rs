//! What is the total of all the name scores in the file? (A name score is its alphabetical
//! position in the list, times the sum of positions of each of its characters in the alphabet.)

use itertools::Itertools;

fn alphabetical_value(s: &str) -> usize {
    s.bytes()
        .filter(u8::is_ascii_alphabetic)
        .map(|c| c.to_ascii_lowercase() - b'a' + 1)
        .map(usize::from)
        .sum()
}

fn solve_for(names: &str) -> usize {
    names
        .split(',')
        .sorted()
        .enumerate()
        .map(|(i, name)| (i + 1) * alphabetical_value(name))
        .sum()
}

super::example!("\"Ellie\",\"Colin\"" => 139);
super::problem!(
    usize:
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/euler/resources/22/names.txt",
        )) => 871_198_282
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabetical_value() {
        assert_eq!(alphabetical_value("Colin"), 53);
        assert_eq!(alphabetical_value("Ellie"), 43);
    }
}
