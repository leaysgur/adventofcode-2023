use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUTS: &str = include_str!("./inputs.txt");

#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
struct Pos {
    z: usize,
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Stack {
    height: usize,
    block: isize,
}

fn parse_inputs(inputs: &str) -> Vec<(Pos, Pos)> {
    let parse_pos = |line: &str| {
        let mut p = line.split(',').map(|i| i.parse::<usize>().unwrap());
        Pos {
            x: p.next().unwrap(),
            y: p.next().unwrap(),
            z: p.next().unwrap(),
        }
    };

    inputs
        .lines()
        .map(|line| {
            let (pos_l, pos_r) = line.split_once('~').unwrap();
            let (pos_l, pos_r) = (parse_pos(pos_l), parse_pos(pos_r));
            (pos_l, pos_r)
        })
        .collect::<Vec<(Pos, Pos)>>()
}

fn main() {
    let parsed = parse_inputs(INPUTS);

    let (mut max_x, mut max_y) = (0, 0);
    let mut blocks = parsed
        .iter()
        .enumerate()
        .map(|(idx, (pos_l, pos_r))| {
            max_x = pos_r.x.max(max_x);
            max_y = pos_r.y.max(max_y);
            Reverse((*pos_l, *pos_r, idx as isize))
        })
        .collect::<BinaryHeap<Reverse<(Pos, Pos, isize)>>>();

    let mut stacks = vec![
        vec![
            Stack {
                height: 0,
                block: -1
            };
            max_y + 1
        ];
        max_x + 1
    ];

    let mut supports = vec![vec![]; blocks.len()];
    let mut supported_by = vec![vec![]; blocks.len()];

    while let Some(rpos) = blocks.pop() {
        let (a, b, idx) = rpos.0;
        let mut max_z: usize = 0;
        for stack_row in stacks.iter().take(b.x + 1).skip(a.x) {
            for stack in stack_row.iter().take(b.y + 1).skip(a.y) {
                max_z = stack.height.max(max_z);
            }
        }

        let new_max_z = max_z + b.z - a.z + 1;

        for stack_row in stacks.iter_mut().take(b.x + 1).skip(a.x) {
            for stack in stack_row.iter_mut().take(b.y + 1).skip(a.y) {
                if stack.height == max_z && stack.block != -1 {
                    if !supports[stack.block as usize].contains(&idx) {
                        supports[stack.block as usize].push(idx)
                    }

                    if !supported_by[idx as usize].contains(&stack.block) {
                        supported_by[idx as usize].push(stack.block);
                    }
                }

                stack.height = new_max_z;
                stack.block = idx;
            }
        }
    }

    let count = supports
        .iter()
        .filter(|v| !v.iter().any(|idx| supported_by[*idx as usize].len() == 1))
        .count();

    println!("Count: {count}");
}
