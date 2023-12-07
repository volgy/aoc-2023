use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

trait HandTrait: Ord {
    fn new(hand: &str) -> Self;
    fn kind(&self) -> HandKind;
}

mod part1;
mod part2;

fn evaluate<T: HandTrait>(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (hand, bid) = s.split_once(" ").unwrap();
            (T::new(hand), bid.parse::<u32>().unwrap())
        })
        .sorted()
        .rev()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank as u32 + 1) * bid)
}

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", evaluate::<part1::Hand>(text));
    println!("Part 2: {}", evaluate::<part2::Hand>(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(evaluate::<part1::Hand>(INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(evaluate::<part2::Hand>(INPUT), 5905);
    }
}
