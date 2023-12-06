const INPUTS: &str = include_str!("./inputs.txt");

fn ways_to_beat(record_time: u32, record_distance: u32) -> u32 {
    let mut beat_count = 0;

    for hold_time in 1..record_time {
        let distance = hold_time * (record_time - hold_time);

        if record_distance < distance {
            beat_count += 1;
        }
    }

    beat_count
}

fn main() {
    let (time_part, distance_part) = INPUTS.split_once('\n').unwrap();

    let times = time_part
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let distances = distance_part
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let races = times.zip(distances).collect::<Vec<_>>();
    // println!("{:?}", races);

    let ways = races
        .iter()
        .map(|(t, d)| ways_to_beat(*t, *d))
        .product::<u32>();
    println!("Ways: {}", ways);
}
