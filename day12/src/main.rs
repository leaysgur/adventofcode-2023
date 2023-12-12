const INPUTS: &str = include_str!("./inputs.txt");

fn count_arrangements(condition: &str, springs: &[usize]) -> usize {
    let line = condition.as_bytes();
    let [n, m] = [line.len(), springs.len()];

    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][springs[m - 1]] = 1;

    for pos in (0..n).rev() {
        for (group, &max_count) in springs.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && springs[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }

        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }

    dp[0][0][0]
}

fn main() {
    let mut total = 0;
    for line in INPUTS.lines() {
        let (condition_part, springs_part) = line.split_once(' ').unwrap();

        let springs = springs_part
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        total += count_arrangements(condition_part, &springs);
    }

    println!("Total: {}", total);
}
