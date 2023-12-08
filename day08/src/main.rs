use std::collections::HashMap;

const INPUTS: &str = include_str!("./inputs.txt");

fn main() {
    let (instructions_part, nodes_part) = INPUTS.split_once("\n\n").unwrap();

    let instructions = instructions_part.chars();

    let mut nodes = HashMap::new();
    nodes_part.lines().for_each(|line| {
        let (src, dest) = line.split_once(" = ").unwrap();
        let (left, right) = dest
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        nodes.insert(src.to_string() + ".L", left);
        nodes.insert(src.to_string() + ".R", right);
    });
    // println!("{:#?}", nodes);

    let mut steps = 0;
    let mut current = "AAA";
    // for dir in instructions {
    for dir in instructions.cycle() {
        let key = current.to_string() + "." + &dir.to_string();
        let next = nodes.get(&key).unwrap();

        println!("{} -> {}", current, next);
        current = next;
        steps += 1;

        if current == "ZZZ" {
            break;
        }
    }

    println!("Steps: {}", steps);
}
