const INPUTS: &str = include_str!("./inputs.txt");

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
    println!("{}", INPUTS);
    let mut grid = INPUTS
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Transpose to modify, North is ←
    grid = transpose(&grid);

    let mut new_grid = vec![vec!['?'; grid.len()]; grid[0].len()];
    for (r_idx, line) in grid.iter().enumerate() {
        let mut new_line = vec!['_'; line.len()];

        let mut s_idx = 0;
        for c_idx in 0..line.len() {
            match line[c_idx] {
                'O' => {
                    if c_idx != s_idx {
                        (new_line[s_idx], new_line[c_idx]) = ('O', '.');
                    } else {
                        new_line[c_idx] = 'O';
                    }
                    s_idx += 1;
                }
                '#' => {
                    new_line[c_idx] = '#';
                    s_idx = c_idx + 1;
                }
                '.' => {
                    new_line[c_idx] = '.';
                }
                _ => unreachable!(),
            }
        }

        new_grid[r_idx] = new_line;
    }

    // Back to original, North is ↑
    new_grid = transpose(&new_grid);

    let rows = new_grid.len();
    let total_load = new_grid.iter().enumerate().map(|(idx, line)| {
        let amount = rows - idx;
        let load = line.iter().filter(|&&c| c == 'O').count();
        // println!("{} => {:>2} x {}", line.iter().collect::<String>(), amount, load);
        (load * amount) as i32
    }).sum::<i32>();

    println!("Total load: {}", total_load);
}
