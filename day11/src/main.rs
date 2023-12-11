use std::collections::HashSet;

const INPUTS: &str = include_str!("./inputs.txt");

type Pos = (usize, usize);

fn calc_steps(
    p1: &Pos,
    p2: &Pos,
    doubled_rows: &HashSet<usize>,
    doubled_cols: &HashSet<usize>,
) -> i32 {
    let mut steps = 0;

    let [mut sorted_x, mut sorted_y] = [[p1.0, p2.0], [p1.1, p2.1]];
    sorted_x.sort_unstable();
    sorted_y.sort_unstable();

    for x in sorted_x[0]..sorted_x[1] {
        steps += 1;

        if doubled_cols.contains(&x) {
            steps += 1;
        }
    }

    for y in sorted_y[0]..sorted_y[1] {
        steps += 1;

        if doubled_rows.contains(&y) {
            steps += 1;
        }
    }

    steps
}

fn main() {
    // println!("{}", INPUTS);

    let mut galaxies = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    for (y, line) in INPUTS.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            row.push(c);

            if c == '#' {
                galaxies.push((x, y));
            }
        }
        grid.push(row);
    }
    let [max_x, max_y] = [grid[0].len(), grid.len()];

    let mut should_expand_rows: HashSet<usize> = HashSet::new();
    let mut should_expand_cols: HashSet<usize> = HashSet::new();
    #[allow(clippy::needless_range_loop)]
    for y in 0..max_y {
        if grid[y].iter().all(|&c| c == '.') {
            should_expand_rows.insert(y);
        }
    }
    for x in 0..max_x {
        if grid.iter().all(|row| row[x] == '.') {
            should_expand_cols.insert(x);
        }
    }

    println!("galaxies: {:?}", galaxies);
    println!("should_expand_rows: {:?}", should_expand_rows);
    println!("should_expand_cols: {:?}", should_expand_cols);

    let mut pairs = 0;
    let mut total_steps = 0;
    let mut seen = HashSet::new();
    for g1 in &galaxies {
        for g2 in &galaxies {
            if g1 == g2 {
                continue;
            }
            if seen.contains(&(g1, g2)) || seen.contains(&(g2, g1)) {
                continue;
            }

            let steps = calc_steps(g1, g2, &should_expand_rows, &should_expand_cols);
            println!("{:?} -> {:?} = {}", g1, g2, steps);
            pairs += 1;
            total_steps += steps;

            seen.insert((g1, g2));
            seen.insert((g2, g1));
        }
    }

    println!("Total steps: {} w/ {} pairs", total_steps, pairs);
}
