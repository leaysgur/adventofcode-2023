use std::collections::{HashMap, VecDeque};

const INPUTS: &str = include_str!("./inputs.txt");

#[derive(Debug)]
enum Module {
    Broadcaster(BroadcasterModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

#[derive(Debug)]
struct BroadcasterModule {
    #[allow(dead_code)]
    name: &'static str,
    dest: Vec<&'static str>,
}

#[derive(Debug)]
struct FlipFlopModule {
    #[allow(dead_code)]
    name: &'static str,
    state: bool,
    dest: Vec<&'static str>,
}

#[derive(Debug)]
struct ConjunctionModule {
    #[allow(dead_code)]
    name: &'static str,
    state: HashMap<&'static str, i32>,
    dest: Vec<&'static str>,
}

#[derive(Debug)]
struct OutputModule {
    #[allow(dead_code)]
    name: &'static str,
}

fn main() {
    let mut modules: HashMap<&str, Module> = HashMap::new();

    for line in INPUTS.lines() {
        let (type_part, dest_part) = line.split_once(" -> ").unwrap();
        let dest = dest_part.split(", ").collect::<Vec<_>>();
        match type_part {
            "broadcaster" => modules.insert(
                type_part,
                Module::Broadcaster(BroadcasterModule {
                    name: type_part,
                    dest: dest.clone(),
                }),
            ),
            _ => {
                let (prefix, name) = (&type_part[..1], &type_part[1..]);
                match prefix {
                    "%" => modules.insert(
                        name,
                        Module::FlipFlop(FlipFlopModule {
                            name,
                            state: false,
                            dest: dest.clone(),
                        }),
                    ),
                    "&" => modules.insert(
                        name,
                        Module::Conjunction(ConjunctionModule {
                            name,
                            state: HashMap::new(),
                            dest: dest.clone(),
                        }),
                    ),
                    _ => unreachable!(),
                }
            }
        };
    }

    // Update ConjunctionModule state
    for line in INPUTS.lines() {
        let (type_part, dest_part) = line.split_once(" -> ").unwrap();

        let name = if type_part == "broadcaster" {
            "broadcaster"
        } else {
            &type_part[1..]
        };

        let dest = dest_part.split(", ").collect::<Vec<_>>();
        for d in &dest {
            if let Some(Module::Conjunction(cm)) = modules.get_mut(d) {
                cm.state.insert(name, 0);
            }
        }
    }
    // println!("{:#?}", modules);

    let mut counter = (0, 0);
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", 0));

        while let Some((from_name, current_name, pulse)) = queue.pop_front() {
            // Count up
            if pulse == 1 {
                counter.1 += 1;
            } else {
                counter.0 += 1;
            }

            println!(
                "{} -{}-> {}",
                from_name,
                if pulse == 1 { "high" } else { "low" },
                current_name,
            );
            let module = modules.get_mut(current_name);

            if module.is_none() {
                continue;
            }

            match module.unwrap() {
                Module::Broadcaster(module) => {
                    for dest in &module.dest {
                        queue.push_back((current_name, dest, pulse));
                    }
                }
                Module::FlipFlop(module) => {
                    if pulse == 1 {
                        continue;
                    }

                    module.state = !module.state;

                    for dest in &module.dest {
                        if module.state {
                            queue.push_back((current_name, dest, 1));
                        } else {
                            queue.push_back((current_name, dest, 0));
                        }
                    }
                }
                Module::Conjunction(module) => {
                    module.state.insert(from_name, pulse);

                    let next_pulse = if module.state.values().all(|&x| x == 1) {
                        0
                    } else {
                        1
                    };

                    for dest in &module.dest {
                        queue.push_back((current_name, dest, next_pulse));
                    }
                }
            }
        }
        println!();
    }

    println!("Total: {:?} => {}", counter, counter.0 * counter.1);
}
