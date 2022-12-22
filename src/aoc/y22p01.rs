/// Solve the problem.
fn solve_for(input: &'static str) -> usize {
    let mut best = [0, 0, 0];

    for elf in input
        .trim_end() // remove trailing `\n`
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|n| n.parse::<usize>().expect("input is valid usizes"))
                .sum()
        })
    {
        if elf > best[0] {
            best[2] = best[1];
            best[1] = best[0];
            best[0] = elf;
        } else if elf > best[1] {
            best[2] = best[1];
            best[1] = elf;
        } else if elf > best[2] {
            best[2] = elf;
        }
    }

    best.iter().sum()
}

super::example!(24000, "22-01");
super::problem!(usize, "22-01");
