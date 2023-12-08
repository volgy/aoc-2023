use std::collections::HashMap;

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", part1(text));
    println!("Part 2: {}", part2(text));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits
            .next()
            .expect("At least one digit should be in the line");
        let last = digits.last().unwrap_or(first);
        sum += 10 * first + last;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let vocab: HashMap<String, u32> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .map(String::from)
    .zip(1..)
    .chain((1..=9).map(|x| (x.to_string(), x)))
    .collect();

    let mut sum = 0;

    for line in input.lines() {
        let mut tail = line;
        let mut digits = vec![];
        while !tail.is_empty() {
            for (name, value) in vocab.iter() {
                if tail.starts_with(name) {
                    digits.push(*value);
                    break;
                }
            }
            // hacky - I hate UTF-8
            let mut chars = tail.chars();
            chars.next();
            tail = chars.as_str();
        }

        assert!(
            !digits.is_empty(),
            "At least one digit should be in the line: {:?}",
            line
        );
        let first = digits[0];
        let last = digits[digits.len() - 1];

        sum += 10 * first + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }
}
