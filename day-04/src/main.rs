use std::collections::{HashMap, HashSet};

fn count_wins(card: &str) -> u64 {
    let (winners, numbers) = card.split_once(":").unwrap().1.split_once("|").unwrap();
    let winners: HashSet<_> = winners.split_whitespace().collect();
    numbers
        .split_whitespace()
        .filter(|n| winners.contains(n))
        .count() as u64
}

fn part1(input: &str) -> u32 {
    let mut value = 0;
    for line in input.lines() {
        let wins = count_wins(line);
        if wins > 0 {
            value += 1 << (wins - 1);
        }
    }
    value
}

// For iterator fan boys
#[allow(dead_code)]
fn part1_with_iterators(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let matches = l
                .split_once(":")
                .unwrap()
                .1
                .split("|")
                .map(|s| s.split_whitespace().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).map(|e| *e).collect())
                .unwrap()
                .len() as u32;
            if matches > 0 {
                1 << matches - 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut deck = HashMap::new();

    for (i_current, line) in input.lines().enumerate() {
        let n_current = deck.entry(i_current).or_insert(0);
        *n_current += 1;
        let multiplier = *n_current;
        let win_range = count_wins(&line) as usize;

        for idx in (i_current + 1)..=(i_current + win_range) {
            *deck.entry(idx).or_insert(0) += multiplier;
        }
    }
    
    deck.values().sum::<u32>()
}

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", part1(text));
    println!("Part 2: {}", part2(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 30);
    }
}
