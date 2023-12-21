use std::collections::HashSet;

const INPUTS: &str = include_str!("./inputs.txt");

type Pos = (i32, i32);

fn print_grid(grid: &[Vec<char>], plots: Option<&HashSet<Pos>>) {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Some(plots) = plots {
                if plots.contains(&(x as i32, y as i32)) {
                    print!("O");
                    count += 1;
                    continue;
                }
            }
            print!("{}", c);
        }
        println!();
    }

    println!("Plots count: {}", count);
}

fn main() {
    let grid = INPUTS
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (max_x, max_y): Pos = (grid[0].len() as i32, grid.len() as i32);
    print_grid(&grid, None);

    let mut start: Pos = (0i32, 0i32);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
                break 'outer;
            }
        }
    }
    println!("Start: {:?}", start);
    println!();

    let mut positions = vec![start];
    let mut plots = HashSet::new();
    for steps in 1..=64 {
        println!("# Steps {}", steps);
        let mut next_items = HashSet::new();
        plots.clear();

        while let Some((x, y)) = positions.pop() {
            println!("## Checking ({}, {})", x, y);

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);

                if nx < 0 || max_x <= nx || ny < 0 || max_y <= ny {
                    continue;
                }

                let c = grid[ny as usize][nx as usize];
                if c == '#' {
                    continue;
                }

                plots.insert((nx, ny));
                next_items.insert((nx, ny));
            }
        }
        println!("## Next: {:?}", next_items);
        positions = next_items.into_iter().collect();
    }

    print_grid(&grid, Some(&plots));
    println!();
}
