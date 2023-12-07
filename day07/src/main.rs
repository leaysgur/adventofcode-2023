use std::cmp::Ordering;
use std::collections::HashMap;

const INPUTS: &str = include_str!("./inputs.txt");

fn detect_type(hand: &[char]) -> (String, u32) {
    let mut counts: HashMap<_, _> = HashMap::new();
    for c in hand {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut dupes = [0; 5];
    for count in counts.values() {
        dupes[*count as usize - 1] += 1;
    }

    match dupes {
        [_, _, _, _, 1] => ("Five of a Kind".to_string(), 6),
        [_, _, _, 1, _] => ("Four of a Kind".to_string(), 5),
        [_, 1, 1, _, _] => ("Full House".to_string(), 4),
        [_, _, 1, _, _] => ("Three of a Kind".to_string(), 3),
        [_, 2, _, _, _] => ("Two Pair".to_string(), 2),
        [_, 1, _, _, _] => ("One Pair".to_string(), 1),
        _ => ("High Card".to_string(), 0),
    }
}

fn hand_compare(a_hand: &[char], b_hand: &[char]) -> Ordering {
    let card_to_score = |card: &char| match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    };

    for (a_card, b_card) in a_hand.iter().zip(b_hand.iter()) {
        let (a_score, b_score) = (card_to_score(a_card), card_to_score(b_card));

        if a_score == b_score {
            continue;
        }
        return a_score.cmp(&b_score);
    }

    Ordering::Equal
}

fn main() {
    let mut hand_and_bids = INPUTS
        .lines()
        .map(|line| {
            let (hand_part, bid_part) = line.split_once(' ').unwrap();

            let hand = hand_part.chars().collect::<Vec<_>>();
            let hand_type = detect_type(hand.as_slice());

            let bid = bid_part.parse::<u32>().unwrap();

            (hand, hand_type, bid)
        })
        .collect::<Vec<_>>();

    hand_and_bids.sort_by(
        |(a_hand, (_, a_type_score), _), (b_hand, (_, b_type_score), _)| {
            a_type_score
                .partial_cmp(b_type_score)
                .unwrap()
                .then(hand_compare(a_hand, b_hand))
        },
    );

    let total = hand_and_bids
        .iter()
        .enumerate()
        .map(|(idx, (_, _, bid))| (idx + 1) as u32 * bid)
        .sum::<u32>();
    println!("Total winnings: {}", total);
}
