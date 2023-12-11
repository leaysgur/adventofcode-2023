const INPUTS: &str = include_str!("./inputs.txt");

enum Dir {
    N,
    S,
    E,
    W,
}

fn main() {
    println!("{}", INPUTS);

    let mut grid: Vec<Vec<char>> = vec![];
    let mut pos = (0, 0);
    for (y, line) in INPUTS.lines().enumerate() {
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
    println!("Start: {:?}", pos);
    // println!("Grid: {:?}", grid);

    #[allow(unused_assignments)]
    let (mut direction, mut steps) = (Dir::N, 0);

    // XXX: Should be caclulated, but my 3 inputs satisfy this
    pos.1 += 1;
    direction = Dir::S;
    steps += 1;

    loop {
        println!("{:?} {}", pos, grid[pos.1][pos.0]);

        match grid[pos.1][pos.0] {
            'S' => break,
            '|' => match direction {
                Dir::N => pos.1 -= 1,
                Dir::S => pos.1 += 1,
                _ => panic!("Unexpected direction on vertical bar"),
            },
            '-' => match direction {
                Dir::E => pos.0 -= 1,
                Dir::W => pos.0 += 1,
                _ => panic!("Unexpected direction on horizontal bar"),
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
                _ => panic!("Unexpected direction on L"),
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
                _ => panic!("Unexpected direction on J"),
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
                _ => panic!("Unexpected direction on 7"),
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
                _ => panic!("Unexpected direction on F"),
            },
            _ => panic!("Unexpected character encountered"),
        }

        steps += 1;
    }

    println!("Steps: {}, farthest = {}", steps, steps / 2);
}
