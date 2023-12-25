use std::collections::{HashMap, HashSet, VecDeque};

const INPUTS: &str = include_str!("./inputs.txt");

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn main() {
    let mut graph: Graph = HashMap::new();

    for line in INPUTS.lines() {
        let (name, components) = line.split_once(": ").unwrap();
        let components = components.split_whitespace().collect::<Vec<_>>();

        // A -> [B, C]
        // A -> B, A -> C
        let set = graph.entry(name).or_default();
        for component in &components {
            set.insert(component);
        }
        // B -> A, C -> A
        for component in &components {
            let set = graph.entry(component).or_default();
            set.insert(name);
        }
    }
    println!("Graph has {} nodes", graph.len());

    // Find the 3 most common edges
    let mut freq = HashMap::new();
    for &start in graph.keys() {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut seen = HashSet::new();
        seen.insert(start);

        while let Some(pos) = queue.pop_front() {
            for &next in &graph[pos] {
                if seen.insert(next) {
                    let key = if pos < next { [pos, next] } else { [next, pos] };

                    let entry = freq.entry(key).or_insert(0);
                    *entry += 1;

                    queue.push_back(next);
                }
            }
        }
    }
    let mut order: Vec<_> = freq.iter().collect();
    order.sort_unstable_by_key(|e| e.1);
    let cut = order.iter().rev().take(3).map(|p| *p.0).collect::<Vec<_>>();

    // Find the size of the largest connected component
    let start = *graph.keys().next().unwrap();

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);

    let mut size = 1;
    while let Some(pos) = queue.pop_front() {
        for &next in &graph[&pos] {
            let key = if pos < next { [pos, next] } else { [next, pos] };

            if cut.contains(&key) {
                continue;
            }

            if seen.insert(next) {
                size += 1;
                queue.push_back(next);
            }
        }
    }

    let (a_size, b_size) = (size, (graph.len() - size));
    println!("Sizes: {} * {} = {}", a_size, b_size, a_size * b_size);
}
