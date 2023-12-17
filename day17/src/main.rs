use std::collections::{BinaryHeap, HashMap};

const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    let grid = INPUTS.lines().map(str::as_bytes).collect::<Vec<_>>();
    let goal = (grid.len() - 1, grid[0].len() - 1);

    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

    while let Some((loss, (r, c, dir))) = queue.pop() {
        if (r, c) == goal {
            println!("Least: {}", -loss);
            break;
        }

        if dists.get(&(r, c, dir)).is_some_and(|&cost| -loss > cost) {
            continue;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if dir == (dr, dc) || dir == (-dr, -dc) {
                continue;
            }

            let mut next_loss = -loss;
            for dist in 1..=3 {
                let nr = (r as isize + dr * dist) as usize;
                let nc = (c as isize + dc * dist) as usize;
                if goal.0 < nr || goal.1 < nc {
                    continue;
                }

                next_loss += (grid[nr][nc] - b'0') as i32;
                let key = (nr, nc, (dr, dc));
                if next_loss < *dists.get(&key).unwrap_or(&i32::MAX) {
                    dists.insert(key, next_loss);
                    queue.push((-next_loss, key));
                }
            }
        }
    }
}
