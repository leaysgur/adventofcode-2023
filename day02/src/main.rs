const INPUTS: &str = include_str!("./inputs.txt");

const TOTAL_RGB: (u32, u32, u32) = (12, 13, 14);

fn main() {
    let mut count = 0;
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    'game: for line in INPUTS.lines() {
        let (l, r) = line.split_once(": ").unwrap();

        let game_id = l.split_whitespace().last().unwrap().parse::<u32>().unwrap();

        for set in r.split("; ") {
            for x in set.split(", ") {
                let (count, color) = x.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();

                if color == "red" && TOTAL_RGB.0 < count {
                    continue 'game;
                }
                if color == "green" && TOTAL_RGB.1 < count {
                    continue 'game;
                }
                if color == "blue" && TOTAL_RGB.2 < count {
                    continue 'game;
                }
            }
        }
        count += game_id;
    }

    println!("Count: {}", count);
}
