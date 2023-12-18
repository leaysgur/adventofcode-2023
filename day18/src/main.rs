const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    // println!("{}", INPUTS);

    let plan = INPUTS
        .lines()
        .map(|line| {
            let mut units = line.split_whitespace();
            let dir = units.next().unwrap().chars().next().unwrap();
            let depth = units.next().unwrap().parse::<i32>().unwrap();
            (dir, depth)
        })
        .collect::<Vec<_>>();

    // XXX: This may be too small in some cases
    const CENTER: i32 = 360;

    let mut grid =
        vec![vec!['.'; (CENTER * 2).try_into().unwrap()]; (CENTER * 2).try_into().unwrap()];

    let mut current = (CENTER, CENTER);
    grid[current.1 as usize][current.0 as usize] = '#';

    for (dir, depth) in plan {
        println!("{} {}", dir, depth);

        match dir {
            'U' => {
                for _ in 1..=depth {
                    current.1 -= 1;
                    grid[current.1 as usize][current.0 as usize] = '#';
                }
            }
            'D' => {
                for _ in 1..=depth {
                    current.1 += 1;
                    grid[current.1 as usize][current.0 as usize] = '#';
                }
            }
            'L' => {
                for _ in 1..=depth {
                    current.0 -= 1;
                    grid[current.1 as usize][current.0 as usize] = '#';
                }
            }
            'R' => {
                for _ in 1..=depth {
                    current.0 += 1;
                    grid[current.1 as usize][current.0 as usize] = '#';
                }
            }
            _ => {}
        }
    }

    println!();
    for line in &grid {
        println!("{}", line.iter().collect::<String>());
    }
    println!();

    // XXX: This position may be already filled in some cases
    let mut stack = vec![(CENTER + 1, CENTER + 1)];
    while let Some(pos) = stack.pop() {
        if pos.0 < 0 || grid[0].len() as i32 <= pos.0 {
            continue;
        }
        if pos.1 < 0 || grid.len() as i32 <= pos.1 {
            continue;
        }
        if grid[pos.1 as usize][pos.0 as usize] == '#' {
            continue;
        }

        grid[pos.1 as usize][pos.0 as usize] = '#';
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            stack.push((pos.0 + dr, pos.1 + dc));
        }
    }

    println!();
    for line in &grid {
        println!("{}", line.iter().collect::<String>());
    }
    println!();

    let total = grid
        .iter()
        .map(|line| line.iter().filter(|c| **c == '#').count() as i32)
        .sum::<i32>();
    println!("Total: {}", total);
}
