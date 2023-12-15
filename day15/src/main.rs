const INPUTS: &str = include_str!("./inputs.txt");
// const INPUTS: &str = "HASH";

fn algo(str: &str) -> u32 {
    let mut current_value = 0;

    for c in str.chars() {
        let ascii = c as u32;
        current_value += ascii;
        current_value *= 17;
        current_value %= 256;
    }

    println!("`{}` = {}", str, current_value);
    current_value
}

fn main() {
    let sum = INPUTS.trim().split(',').map(algo).sum::<u32>();
    println!("Sum: {}", sum);
}
