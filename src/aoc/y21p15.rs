use std::collections::BinaryHeap;

type Grid = Vec<Vec<u32>>;

fn parse(input: &'static str) -> Grid {
    input
        .trim_end()
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("input is valid u32s"))
                .collect()
        })
        .collect()
}

/* fn expand(g: &Grid, factor: usize) -> Grid {
    (0..(factor * g.len()))
        .map(|x| {
            (0..(factor * g[0].len()))
                .map(|y| {
                    (g[x % g.len()][y % g[0].len()]
                        + u32::try_from(x / g.len() + y / g[0].len())
                            .expect("grid is not 2^32 long"))
                        % 10
                })
                .collect()
        })
        .collect()
} */

fn expand(g: &Grid, factor: usize) -> Grid {
    (0..(factor * g.len()))
        .map(|x| {
            (0..(factor * g[0].len()))
                .map(|y| {
                    let cost = g[x % g.len()][y % g[0].len()]
                        // apply the increment
                        + u32::try_from(x / g.len() + y / g[0].len())
                            .expect("grid is not 2^32 long");

                    // apply wrapping: 10 -> 1, hence the extra cost / 10
                    cost % 10 + cost / 10
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Default, PartialEq, Eq)]
struct DijkstraState {
    position: (usize, usize),
    cost: u32,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // smaller costs compare higher, since BinaryHeap is a max heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Get values of adjacent elements. Diagonals are not adjacent.
fn neighbors(g: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if x > 0 {
        out.push((x - 1, y));
    }
    if x < g.len() - 1 {
        out.push((x + 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if y < g[x].len() - 1 {
        out.push((x, y + 1));
    }
    out
}

fn dijkstra(grid: &Grid) -> u32 {
    let goal = (grid.len() - 1, grid[0].len() - 1);

    let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::from([DijkstraState::default()]);

    while let Some(DijkstraState {
        cost,
        position: (x, y),
    }) = queue.pop()
    {
        if (x, y) == goal {
            return cost;
        }
        if cost > dist[x][y] {
            continue;
        }
        for (nx, ny) in neighbors(grid, x, y) {
            let ncost = cost + grid[nx][ny];
            if ncost < dist[nx][ny] {
                queue.push(DijkstraState {
                    position: (nx, ny),
                    cost: ncost,
                });
                dist[nx][ny] = ncost;
            }
        }
    }

    unreachable!("grid is well-connected")
}

fn solve_for(input: &'static str) -> u32 {
    dijkstra(&expand(&parse(input), 5))
}

super::example!(315, "21-15");
super::problem!(u32, "21-15");
