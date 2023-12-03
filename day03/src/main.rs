const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    let grid: Vec<Vec<char>> = INPUTS.lines().map(|line| line.chars().collect()).collect();
    let [grid_height, grid_width] = [grid.len(), grid[0].len()];

    let mut sum = 0;
    for row in 0..grid_height {
        let mut col = 0;
        while col < grid_width {
            if grid[row][col].is_ascii_digit() {
                let start_col = col;

                // Eat the number
                let mut num = 0;
                while col < grid_width && grid[row][col].is_ascii_digit() {
                    num = num * 10 + (grid[row][col] as u8 - b'0') as i32;
                    col += 1;
                }
                let end_col = col - 1;

                let mut is_adjacent = false;
                'adjacent_check: for d_col in (start_col as isize - 1)..=(end_col as isize + 1) {
                    for d_row in (row as isize - 1)..=(row as isize + 1) {
                        // Skip out of bound
                        if !(0 <= d_row
                            && d_row < grid_height as isize
                            && 0 <= d_col
                            && d_col < grid_width as isize)
                        {
                            continue;
                        }

                        let dc = grid[d_row as usize][d_col as usize];
                        if dc != '.' && !dc.is_ascii_digit() {
                            is_adjacent = true;
                            break 'adjacent_check;
                        }
                    }
                }

                if is_adjacent {
                    sum += num;
                }
            } else {
                col += 1;
            }
        }
    }

    println!("Sum: {}", sum);
}
