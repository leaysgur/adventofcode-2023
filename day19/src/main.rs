use std::collections::HashMap;

const INPUTS: &str = include_str!("./inputs.txt");

fn parse_ratings(ratings_part: &str) -> Ratings {
    let mut ratings = ratings_part
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',');
    let x = ratings
        .next()
        .unwrap()
        .trim_start_matches("x=")
        .parse::<u32>()
        .unwrap();
    let m = ratings
        .next()
        .unwrap()
        .trim_start_matches("m=")
        .parse::<u32>()
        .unwrap();
    let a = ratings
        .next()
        .unwrap()
        .trim_start_matches("a=")
        .parse::<u32>()
        .unwrap();
    let s = ratings
        .next()
        .unwrap()
        .trim_start_matches("s=")
        .parse::<u32>()
        .unwrap();

    Ratings { x, m, a, s }
}

#[derive(Debug)]
struct Ratings {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

type Workflows = HashMap<String, Vec<String>>;

fn parse_workflows(workflows_part: &str) -> Workflows {
    let mut workflows: Workflows = HashMap::new();

    for line in workflows_part.lines() {
        let (name, flows) = line.trim_end_matches('}').split_once('{').unwrap();
        workflows.insert(
            name.to_string(),
            flows.split(',').map(|s| s.to_string()).collect(),
        );
    }

    workflows
}

fn eval(cond: &str, ratings: &Ratings) -> Option<String> {
    let (stmt_part, next) = cond.split_once(':').unwrap();

    let mut stmt = stmt_part.chars();

    let lhs = stmt.next().unwrap();
    let op = stmt.next().unwrap();
    let rhs = stmt.collect::<String>().parse::<u32>().unwrap();

    println!("  {} {} {} ? {}", lhs, op, rhs, next);

    match (lhs, op) {
        ('x', '>') => {
            if ratings.x > rhs {
                return Some(next.to_string());
            }
        }
        ('x', '<') => {
            if ratings.x < rhs {
                return Some(next.to_string());
            }
        }
        ('m', '>') => {
            if ratings.m > rhs {
                return Some(next.to_string());
            }
        }
        ('m', '<') => {
            if ratings.m < rhs {
                return Some(next.to_string());
            }
        }
        ('a', '>') => {
            if ratings.a > rhs {
                return Some(next.to_string());
            }
        }
        ('a', '<') => {
            if ratings.a < rhs {
                return Some(next.to_string());
            }
        }
        ('s', '>') => {
            if ratings.s > rhs {
                return Some(next.to_string());
            }
        }
        ('s', '<') => {
            if ratings.s < rhs {
                return Some(next.to_string());
            }
        }
        _ => unreachable!(),
    }

    None
}

fn run_workflow(ratings: &Ratings, workflows: &Workflows) -> u32 {
    let mut current = "in".to_string();
    'flow: loop {
        println!("Current: {} w/ {:?}", current, ratings);

        if current == "A" {
            return ratings.x + ratings.m + ratings.a + ratings.s;
        }
        if current == "R" {
            return 0;
        }

        let mut flows = workflows.get(&current).unwrap().clone();
        println!("Flows: {:?}", flows);
        let last = flows.pop().unwrap();
        for cond in flows {
            println!("  Cond: {:?}", cond);
            if let Some(res) = eval(&cond, ratings) {
                current = res;
                continue 'flow;
            }
        }
        current = last;
    }
}

fn main() {
    let (workflows_part, ratings_part) = INPUTS.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows_part);
    // println!("Workflows: {:?}", workflows);

    let mut total = 0;
    for line in ratings_part.lines() {
        let ratings = parse_ratings(line);
        let numbers = run_workflow(&ratings, &workflows);
        total += numbers;
        println!();
    }
    println!("Total: {}", total);
}
