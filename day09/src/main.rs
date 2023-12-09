const INPUTS: &str = include_str!("./inputs.txt");

fn predict(history: Vec<i64>) -> i64 {
    let mut value = *history.last().unwrap();
    let mut current = history.clone();
    loop {
        let step = current
            .windows(2)
            .map(|w| (w[1] - w[0]))
            .collect::<Vec<_>>();

        let v = *step.last().unwrap();
        value += v;

        // Early exit at last 2!
        // Last 3 = ? ? ? ?
        // Last 2 =  N N N
        // Last 1 =   0 0
        if step.iter().all(|&x| x == v) {
            break;
        }

        current = step.clone();
    }

    value
}

fn main() {
    println!("{}", INPUTS);

    let histories = INPUTS.lines().map(|line| {
        line.split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    });

    let sum = histories.map(predict).sum::<i64>();
    println!("Sum: {}", sum);
}
