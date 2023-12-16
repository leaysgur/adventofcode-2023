use std::collections::HashSet;

const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    println!("{}", INPUTS);

    let grid = INPUTS
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let (max_rows, max_cols) = (grid.len() as i32, grid[0].len() as i32);

    let mut seen = HashSet::new();
    let mut energized = HashSet::new();
    let mut rays = vec![];

    rays.push(((0, 0), 'R'));

    while let Some(item) = rays.pop() {
        let ((r, c), dir) = item;

        if r < 0 || c < 0 || max_rows <= r || max_cols <= c {
            continue;
        }
        if seen.contains(&((r, c), dir)) {
            continue;
        }

        energized.insert((r, c));
        seen.insert(((r, c), dir));

        let tile = grid[r as usize][c as usize];
        // println!("item: {:?}, tile: {}", (r,c, dir), tile);
        match (dir, tile) {
            ('R', '.' | '-') => {
                rays.push(((r, c + 1), 'R'));
            }
            ('L', '.' | '-') => {
                rays.push(((r, c - 1), 'L'));
            }
            ('T' | 'B', '-') => {
                rays.push(((r, c - 1), 'L'));
                rays.push(((r, c + 1), 'R'));
            }
            ('T', '.' | '|') => {
                rays.push(((r - 1, c), 'T'));
            }
            ('B', '.' | '|') => {
                rays.push(((r + 1, c), 'B'));
            }
            ('R' | 'L', '|') => {
                rays.push(((r - 1, c), 'T'));
                rays.push(((r + 1, c), 'B'));
            }
            ('R', '/') | ('L', '\\') => {
                rays.push(((r - 1, c), 'T'));
            }
            ('L', '/') | ('R', '\\') => {
                rays.push(((r + 1, c), 'B'));
            }
            ('T', '/') | ('B', '\\') => {
                rays.push(((r, c + 1), 'R'));
            }
            ('B', '/') | ('T', '\\') => {
                rays.push(((r, c - 1), 'L'));
            }
            _ => unreachable!(),
        }
    }

    for rr in 0..max_rows {
        for cc in 0..max_cols {
            if energized.contains(&(rr, cc)) {
                print!("#");
            } else {
                print!("{}", &grid[rr as usize][cc as usize])
            }
        }
        println!();
    }
    println!("energized: {:?}", energized.len())
}
