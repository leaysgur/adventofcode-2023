const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    let sum = INPUTS
        .lines()
        .map(extract_numbers)
        .map(|(first, last)| (first * 10) + last)
        .sum::<u32>();

    println!("Sum: {}", sum);
}

fn extract_numbers(line: &str) -> (u32, u32) {
    let [mut first, mut last] = [None, None];

    for c in line.chars() {
        if c.is_numeric() {
            first = Some(c.to_digit(10).unwrap());
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_numeric() {
            last = Some(c.to_digit(10).unwrap());
            break;
        }
    }

    (first.unwrap(), last.unwrap())
}
