#![allow(dead_code)]

struct Probe {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Probe {
    fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.velocity.0 += if self.velocity.0 > 0 { -1 } else { 1 };
        self.velocity.1 -= 1;
    }
}

#[derive(Debug)]
struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Target {
    fn from_iter(mut iter: impl Iterator<Item = i32>) -> Self {
        Self {
            xmin: iter.next().unwrap(),
            xmax: iter.next().unwrap(),
            ymin: iter.next().unwrap(),
            ymax: iter.next().unwrap(),
        }
    }
}

fn solve_for(input: &'static str) -> i32 {
    let target = Target::from_iter(
        input
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .flat_map(str::parse),
    );

    // sum over the integers up to ymin, because we lose one unit velocity per step, so we move by
    // each unit of velocity along the way
    (target.ymin + 1) * target.ymin / 2
}

super::example!(45, "17");
super::problem!(i32, "17");
