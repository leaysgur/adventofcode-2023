const INPUTS: &str = include_str!("./inputs.txt");
const X: i64 = 200_000_000_000_000;
const Y: i64 = 400_000_000_000_000;
// const X: i64 = 7;
// const Y: i64 = 27;

type Pos = (i64, i64, i64);
type HailStone = (Pos, Pos);

fn parse_inputs(inputs: &str) -> Vec<HailStone> {
    let parse_pos = |line: &str| {
        let mut p = line.split(',');
        (
            p.next().unwrap().trim().parse::<i64>().unwrap(),
            p.next().unwrap().trim().parse::<i64>().unwrap(),
            p.next().unwrap().trim().parse::<i64>().unwrap(),
        )
    };

    inputs
        .lines()
        .map(|s| {
            let (pos, vel) = s.split_once(" @ ").unwrap();
            (parse_pos(pos), parse_pos(vel))
        })
        .collect()
}

fn check_intersection(
    ((ax, ay, _), (avx, avy, _)): HailStone,
    ((bx, by, _), (bvx, bvy, _)): HailStone,
) -> Option<Pos> {
    let linear = |(a1, b1, c1), (a2, b2, c2)| {
        let det = (a1 * b2) - (a2 * b1);
        ((b2 * c1 - b1 * c2) / det, (a1 * c2 - a2 * c1) / det)
    };

    let (s, t) = linear(
        (avx as f64, -bvx as f64, (bx - ax) as f64),
        (avy as f64, -bvy as f64, (by - ay) as f64),
    );

    if s < 0.0 || t < 0.0 {
        return None;
    }

    let (xs, ys) = (ax as f64 + avx as f64 * s, ay as f64 + avy as f64 * s);
    let (xt, yt) = (bx as f64 + bvx as f64 * t, by as f64 + bvy as f64 * t);
    let (x, y) = ((xs + xt) / 2.0, (ys + yt) / 2.0);

    Some((x as i64, y as i64, 0))
}

fn main() {
    let hail_stones = parse_inputs(INPUTS);

    let mut count = 0;
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let (hail_stone1, hail_stone2) = (hail_stones[i], hail_stones[j]);
            if let Some((x, y, _)) = check_intersection(hail_stone1, hail_stone2) {
                if (X..=Y).contains(&x) && (X..=Y).contains(&y) {
                    count += 1;
                }
            }
        }
    }
    println!("Count: {count}");
}
