use std::collections::HashSet;

const INPUTS: &str = include_str!("./inputs.txt");

type Pos = (i32, i32);
#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    U,
    R,
    L,
    D,
}

fn find_start_and_goal(grid: &[Vec<char>]) -> (Pos, Pos) {
    let start_x = grid
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, &c)| c == '.')
        .unwrap();
    let goal_x = grid
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, &c)| c == '.')
        .unwrap();
    (
        (start_x.0 as i32, 0),
        (goal_x.0 as i32, grid.len() as i32 - 1),
    )
}

fn main() {
    let grid = INPUTS
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (max_x, max_y): Pos = (grid[0].len() as i32, grid.len() as i32);

    let (start, goal) = find_start_and_goal(&grid);
    println!("S:{start:?} => G:{goal:?}");
    println!();

    let mut goal_steps = vec![];
    let mut queue = vec![(start, HashSet::new(), 0)];
    while let Some(((x, y), mut seen, mut steps)) = queue.pop() {
        if (x, y) == goal {
            println!("Goal with {steps} steps!");
            goal_steps.push(steps);
        }

        seen.insert((x, y));
        steps += 1;

        for ((dx, dy), dir) in [
            ((0, 1), Dir::D),
            ((1, 0), Dir::R),
            ((-1, 0), Dir::L),
            ((0, -1), Dir::U),
        ] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || max_x <= nx || ny < 0 || max_y <= ny {
                continue;
            }
            if seen.contains(&(nx, ny)) {
                continue;
            }

            let nc = grid[ny as usize][nx as usize];
            match nc {
                '#' => {
                    continue;
                }
                '.' => {
                    queue.push(((nx, ny), seen.clone(), steps));
                }
                '>' => {
                    if dir != Dir::R {
                        continue;
                    }
                    queue.push(((nx, ny), seen.clone(), steps));
                }
                'v' => {
                    if dir != Dir::D {
                        continue;
                    }
                    queue.push(((nx, ny), seen.clone(), steps));
                }
                _ => unreachable!(),
            }
        }
    }

    goal_steps.sort_unstable();
    println!();
    println!("Max steps: {:?}", goal_steps.last().unwrap());
}
