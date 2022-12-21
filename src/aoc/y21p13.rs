use std::{collections::HashSet, fmt::Display};

/// Solve the problem.
#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

struct Sheet {
    dots: HashSet<(usize, usize)>,
}

impl Sheet {
    fn fold(&mut self, fold: Fold) {
        let dots = std::mem::take(&mut self.dots);
        self.dots = dots
            .into_iter()
            .map(|(x, y)| match fold {
                Fold::X(line) => {
                    if x > line {
                        (2 * line - x, y)
                    } else {
                        (x, y)
                    }
                }
                Fold::Y(line) => {
                    if y > line {
                        (x, 2 * line - y)
                    } else {
                        (x, y)
                    }
                }
            })
            .collect();
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.dots.iter().map(|(x, _)| x).max().unwrap_or(&0) + 1;
        let height = self.dots.iter().map(|(_, y)| y).max().unwrap_or(&0) + 1;

        let mut out = String::with_capacity(height * (width + 1));

        for y in 0..height {
            for x in 0..width {
                if self.dots.contains(&(x, y)) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }

        write!(f, "{out}")
    }
}

fn to_dot(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').expect("input is well-formed");
    (
        x.parse().expect("input is valid usizes"),
        y.parse().expect("input is valid usizes"),
    )
}

fn to_fold(s: &str) -> Fold {
    let mut s = s.split(' ').nth(2).expect("folds are well-formed").chars();
    let fold = match s.next().expect("folds have a direction") {
        'x' => Fold::X,
        'y' => Fold::Y,
        unknown => panic!("invalid fold direction: {unknown}"),
    };
    let n = s
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("folds have a coordinate");
    fold(n)
}

fn solve_for(input: &'static str) -> String {
    let (dots, folds) = input.split_once("\n\n").expect("input is well-formed");

    let mut sheet = Sheet {
        dots: dots.split('\n').map(to_dot).collect(),
    };

    for fold in folds.trim_end().split('\n').map(to_fold) {
        sheet.fold(fold);
    }

    sheet.to_string()
}

super::example!(
    "#####
#...#
#...#
#...#
#####
",
    "21-13"
);
super::problem!(String, "21-13");
