const INPUTS: &str = include_str!("./inputs.txt");

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

enum Dir {
    N,
    S,
    E,
    W,
}

fn parse(inputs: &str) -> (Grid, Pos) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut pos = (0, 0);
    for (y, line) in inputs.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            row.push(c);

            // println!("[{}][{}] = {}", x, y, c);
            if c == 'S' {
                pos = (x, y);
            }
        }
        grid.push(row);
    }

    (grid, pos)
}

fn initial_step(grid: &Grid, pos: Pos) -> (Dir, Pos) {
    match grid[pos.1 + 1][pos.0] {
        '|' | 'J' | 'L' => return (Dir::S, (pos.0, pos.1 + 1)),
        _ => (),
    }
    match grid[pos.1 - 1][pos.0] {
        '|' | '7' | 'F' => return (Dir::N, (pos.0, pos.1 - 1)),
        _ => (),
    }
    match grid[pos.1][pos.0 + 1] {
        '-' | 'J' | '7' => return (Dir::W, (pos.0 + 1, pos.1)),
        _ => (),
    }
    match grid[pos.1][pos.0 - 1] {
        '-' | 'L' | 'F' => return (Dir::E, (pos.0 - 1, pos.1)),
        _ => (),
    }

    unreachable!();
}

fn main() {
    println!("{}", INPUTS);

    let (grid, start) = parse(INPUTS);
    println!("Start: {:?}", start);
    // println!("Grid: {:?}", grid);

    let mut steps = 0;

    let (mut direction, mut pos) = initial_step(&grid, start);
    steps += 1;

    loop {
        println!("{:?} {}", pos, grid[pos.1][pos.0]);

        match grid[pos.1][pos.0] {
            'S' => break,
            '|' => match direction {
                Dir::N => pos.1 -= 1,
                Dir::S => pos.1 += 1,
                _ => unreachable!(),
            },
            '-' => match direction {
                Dir::E => pos.0 -= 1,
                Dir::W => pos.0 += 1,
                _ => unreachable!(),
            },
            'L' => match direction {
                Dir::S => {
                    pos.0 += 1;
                    direction = Dir::W
                }
                Dir::E => {
                    pos.1 -= 1;
                    direction = Dir::N
                }
                _ => unreachable!(),
            },
            'J' => match direction {
                Dir::S => {
                    pos.0 -= 1;
                    direction = Dir::E
                }
                Dir::W => {
                    pos.1 -= 1;
                    direction = Dir::N
                }
                _ => unreachable!(),
            },
            '7' => match direction {
                Dir::N => {
                    pos.0 -= 1;
                    direction = Dir::E
                }
                Dir::W => {
                    pos.1 += 1;
                    direction = Dir::S
                }
                _ => unreachable!(),
            },
            'F' => match direction {
                Dir::N => {
                    pos.0 += 1;
                    direction = Dir::W
                }
                Dir::E => {
                    pos.1 += 1;
                    direction = Dir::S
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        steps += 1;
    }

    println!("Steps: {}, farthest = {}", steps, steps / 2);
}
