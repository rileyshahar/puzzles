use std::collections::HashMap;

use itertools::Itertools;

struct Polymer(String);

impl Polymer {
    fn apply(self, rules: &HashMap<(char, char), char>) -> Self {
        let mut out = String::new();
        for (a, b) in self.0.chars().tuple_windows() {
            out.push(a);
            if let Some(&c) = rules.get(&(a, b)) {
                out.push(c);
            }
        }
        // probably an easier way to do this, but we don't get the last character from the loop
        out.push(self.0.chars().rev().next().unwrap());
        Self(out)
    }
}

struct InsertionRule((char, char), char);

impl InsertionRule {
    fn from_string(s: &'static str) -> Self {
        let (from, to) = s.split_once(" -> ").expect("a polymer rule has a ->");
        let mut from = from.chars();
        Self(
            (from.next().unwrap(), from.next().unwrap()),
            to.chars().next().unwrap(),
        )
    }
}

/// Solve the problem.
fn solve_for(input: &'static str) -> usize {
    let (template, rules) = input
        .trim_end() // remove trailing `\n`
        .split_once("\n\n")
        .expect("input is well-formed");

    let mut polymer = Polymer(template.into());
    let rules: HashMap<_, _> = rules
        .split('\n')
        .map(InsertionRule::from_string)
        .map(|i| (i.0, i.1))
        .collect();

    for _ in 0..40 {
        polymer = polymer.apply(&rules);
    }

    let counts = polymer.0.chars().counts();
    let mut sorted = counts.iter().sorted_by_key(|(_, n)| *n);
    let least = sorted.next().unwrap();
    sorted.last().unwrap().1 - least.1
}

super::example!(1588, "14");
super::problem!(usize, "14");
