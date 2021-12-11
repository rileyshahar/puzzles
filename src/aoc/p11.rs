use std::collections::VecDeque;

#[derive(Debug)]
struct Octopus {
    flashed: bool,
    energy: u32,
}

impl Octopus {
    const fn new(energy: u32) -> Self {
        Self {
            energy,
            flashed: false,
        }
    }

    fn increment(&mut self) {
        self.energy += 1;
    }

    fn flash(&mut self) {
        self.flashed = true;
        self.energy = 0;
    }

    /// Perform a step, returning whether self should flash.
    fn step(&mut self) -> bool {
        if !self.flashed {
            self.increment();
            if self.energy > 9 {
                self.flash();
                return true;
            }
        }
        false
    }
}

type Grid = Vec<Vec<Octopus>>;

fn neighbors(
    x: isize,
    y: isize,
    x_lim: isize,
    y_lim: isize,
) -> impl Iterator<Item = (usize, usize)> {
    #[allow(clippy::cast_sign_loss)] // checked first that cast is valid
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter(move |(dx, dy)| x + dx >= 0 && x + dx < x_lim && y + dy >= 0 && y + dy < y_lim)
    .map(move |(dx, dy)| ((x + dx) as usize, (y + dy) as usize))
}

fn step(g: &mut Grid) -> u32 {
    let mut queue = VecDeque::new();
    let mut flashes = 0;
    for x in 0..g.len() {
        for y in 0..g[0].len() {
            if g[x][y].step() {
                queue.push_back((x, y));
                flashes += 1;
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        #[allow(clippy::cast_possible_wrap)]
        for (nx, ny) in neighbors(
            x as isize,
            y as isize,
            g.len() as isize,
            g[0].len() as isize,
        ) {
            if g[nx][ny].step() {
                queue.push_back((nx, ny));
                flashes += 1;
            }
        }
    }

    for o in g.iter_mut().flat_map(|row| row.iter_mut()) {
        o.flashed = false;
    }

    flashes
}

fn display(g: &Grid) -> String {
    let mut out = String::new();
    for row in g {
        for entry in row {
            out += &entry.energy.to_string();
        }
        out += "\n";
    }
    out
}

fn solve_for(input: &'static str) -> u32 {
    let mut grid: Grid = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| Octopus::new(c.to_digit(10).expect("input is valid u32s")))
                .collect()
        })
        .collect();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }
    flashes
}

super::example!(1656, "11");
super::problem!(u32, "11");
