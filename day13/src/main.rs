const INPUTS: &str = include_str!("./inputs.txt");

fn check(grid: &[Vec<char>]) -> u32 {
    let (height, width) = (grid.len(), grid[0].len());

    'check: for y in 0..height - 1 {
        for (y0, y1) in (0..=y).rev().zip(y + 1..height) {
            for x in 0..width {
                if grid[y0][x] != grid[y1][x] {
                    continue 'check;
                }
            }
        }

        return y as u32 + 1;
    }

    0
}

fn transpose(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = vec![vec![' '; pattern.len()]; pattern[0].len()];
    for (idx, line) in pattern.iter().enumerate() {
        for (idx2, ch) in line.iter().enumerate() {
            result[idx2][idx] = *ch;
        }
    }
    result
}

fn main() {
    let mut sum = 0;
    for pattern in INPUTS.split("\n\n") {
        println!("{}", pattern);
        let parsed = pattern
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        sum += dbg!(check(&parsed) * 100);
        sum += dbg!(check(&transpose(&parsed)));
    }

    println!("Sum: {}", sum);
}
