use std::collections::HashMap;

use itertools::Itertools;

struct Polymer {
    map: HashMap<(char, char), usize>,

    // the map will double-count everything else, so we keep track of these
    start: char,
    end: char,
}

impl Polymer {
    fn apply(self, rules: &HashMap<(char, char), char>) -> Self {
        let mut map = HashMap::new();
        for (&(a, b), &n) in &self.map {
            if let Some(&c) = rules.get(&(a, b)) {
                let count = map.entry((a, c)).or_insert(0);
                *count += n;

                let count = map.entry((c, b)).or_insert(0);
                *count += n;
            } else {
                let count = map.entry((a, b)).or_insert(0);
                *count += n;
            }
        }
        Self { map, ..self }
    }
}

fn parse_insertion_rule(s: &'static str) -> ((char, char), char) {
    let (from, to) = s.split_once(" -> ").expect("a polymer rule has a ->");
    let mut from = from.chars();
    (
        (from.next().unwrap(), from.next().unwrap()),
        to.chars().next().unwrap(),
    )
}

/// Solve the problem.
fn solve_for(input: &'static str) -> usize {
    let (template, rules) = input
        .trim_end() // remove trailing `\n`
        .split_once("\n\n")
        .expect("input is well-formed");

    let start = template.chars().next().unwrap();
    let end = template.chars().last().unwrap();
    let mut polymer = Polymer {
        start,
        end,
        map: template.chars().tuple_windows().counts(),
    };

    let rules = rules.split('\n').map(parse_insertion_rule).collect();

    for _ in 0..40 {
        polymer = polymer.apply(&rules);
    }

    let counts = polymer.map.into_iter().fold(
        HashMap::from([(polymer.end, 1), (polymer.start, 1)]),
        |mut counts, ((s, e), n)| {
            let count = counts.entry(s).or_insert(0);
            *count += n;

            let count = counts.entry(e).or_insert(0);
            *count += n;

            counts
        },
    );

    let mut sorted = counts.iter().sorted_by_key(|(_, n)| *n);
    let least = sorted.next().unwrap();

    // we double counted everything
    (sorted.last().unwrap().1 - least.1) / 2
}

super::example!(2_188_189_693_529, "21-14");
super::problem!(usize, "21-14");
