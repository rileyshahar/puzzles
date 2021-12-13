use std::collections::HashSet;

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
            .map(|dot| match fold {
                Fold::X(amt) => {
                    if dot.0 > amt {
                        (amt - (dot.0 - amt), dot.1)
                    } else {
                        dot
                    }
                }
                Fold::Y(amt) => {
                    if dot.1 > amt {
                        (dot.0, amt - (dot.1 - amt))
                    } else {
                        dot
                    }
                }
            })
            .collect();
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
    let fold = match s.next().unwrap() {
        'x' => Fold::X,
        'y' => Fold::Y,
        _ => unreachable!(),
    };
    let n = s.skip(1).collect::<String>().parse().unwrap();
    fold(n)
}

fn solve_for(input: &'static str) -> usize {
    let (dots, folds) = input.split_once("\n\n").expect("input is well-formed");

    let mut sheet = Sheet {
        dots: dots
            .trim_end() // remove trailing `\n`
            .split('\n')
            .map(to_dot)
            .collect(),
    };

    sheet.fold(folds.split('\n').map(to_fold).next().unwrap());

    sheet.dots.len()
}

super::example!(17, "13");
super::problem!(usize, "13");
