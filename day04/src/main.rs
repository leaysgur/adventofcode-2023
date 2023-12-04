use std::collections::HashSet;

const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    let mut sum = 0;
    for card in INPUTS.lines() {
        let (_, numbers) = card.split_once(": ").unwrap();
        let (winning_numbers_part, your_numbers_part) = numbers.split_once(" | ").unwrap();

        let winning_numbers: HashSet<u8> = winning_numbers_part
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let your_numbers: Vec<u8> = your_numbers_part
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut count = 0;
        for n in &your_numbers {
            if winning_numbers.contains(n) {
                count += 1;
            }
        }

        let score = if 0 < count { (2_i32).pow(count - 1) } else { 0 };
        // println!("{:?} vs {:?} => {}", winning_numbers, your_numbers, score);

        sum += score;
    }

    println!("Sum: {}", sum);
}
