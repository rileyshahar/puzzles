#[derive(PartialEq)]
enum CaveSize {
    Large,
    Small,
}

struct Cave {
    size: CaveSize,
    adjacency: Vec<usize>,
    name: &'static str,
}

impl Cave {
    fn from_str(name: &'static str) -> Self {
        Self {
            size: if name.starts_with(char::is_lowercase) {
                CaveSize::Small
            } else {
                CaveSize::Large
            },
            adjacency: vec![],
            name,
        }
    }
}

#[derive(Default)]
struct Caves {
    caves: Vec<Cave>,
}

impl Caves {
    fn find(&self, s: &str) -> Option<usize> {
        self.caves.iter().position(|c| c.name == s)
    }

    fn get_or_insert(&mut self, s: &'static str) -> usize {
        self.find(s).unwrap_or_else(|| {
            let new = Cave::from_str(s);
            self.caves.push(new);
            self.caves.len() - 1
        })
    }

    fn add_connection(&mut self, from: usize, to: usize) {
        self.caves[from].adjacency.push(to);
        self.caves[to].adjacency.push(from);
    }

    fn dfs(&self, visit: &mut impl FnMut(&Cave) -> bool) {
        self.dfs_step(
            self.find("start").expect("a start always exists"),
            &mut vec![0; self.caves.len()],
            visit,
        );
    }

    fn visited_small_twice(&self, visits: &[usize]) -> bool {
        self.caves
            .iter()
            .enumerate()
            .filter(|(_, c)| c.size == CaveSize::Small)
            .map(|(i, _)| visits[i])
            .any(|visits| visits > 1)
    }

    fn dfs_step(
        &self,
        start: usize,
        visits: &mut Vec<usize>,
        visit: &mut impl FnMut(&Cave) -> bool,
    ) {
        visits[start] += 1;
        if visit(&self.caves[start]) {
            for &cave in &self.caves[start].adjacency {
                if visits[cave] < 1
                    || self.caves[cave].size == CaveSize::Large
                    || !self.visited_small_twice(visits)
                        && self.caves[cave].name != "start"
                        && self.caves[cave].name != "end"
                {
                    self.dfs_step(cave, visits, visit);
                }
            }
        }
        visits[start] -= 1;
    }
}

fn solve_for(input: &'static str) -> usize {
    let mut caves = Caves::default();

    for line in input
        .trim_end() // remove trailing `\n`
        .split('\n')
    {
        let (left, right) = line.split_once('-').expect("input is well-formed");

        let left_index = caves.get_or_insert(left);
        let right_index = caves.get_or_insert(right);

        caves.add_connection(left_index, right_index);
    }

    let mut counter = 0;
    caves.dfs(&mut |cave| {
        if cave.name == "end" {
            counter += 1;
            false
        } else {
            true
        }
    });

    counter
}

super::example!(3509, "21-12");
super::problem!(usize, "21-12");
