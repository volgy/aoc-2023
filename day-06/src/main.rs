fn parse1(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();

    let mut get_items = |prefix| {
        lines
            .next()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
    };

    get_items("Time: ").zip(get_items("Distance: ")).collect()
}

fn parse2(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let mut get_item = |prefix| {
        lines
            .next()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    };

    (get_item("Time: "), get_item("Distance: "))
}

fn margin(time: u64, distance: u64) -> u64 {
    let d_roots = (((time * time) - 4 * distance) as f64).sqrt();
    let lo = (time as f64 - d_roots) / 2.0 * (1.0 + f64::EPSILON);
    let hi = (time as f64 + d_roots) / 2.0 * (1.0 - f64::EPSILON);
    if d_roots > 0.0 {
        (hi.floor() - lo.ceil() + 1.0) as u64
    } else {
        0
    }
}

fn part1(input: &str) -> u64 {
    parse1(input)
        .into_iter()
        .map(|(t, d)| margin(t, d))
        .product()
}

fn part2(input: &str) -> u64 {
    let (time, distance) = parse2(input);
    margin(time, distance)
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
        Time:      7  15   30\n\
        Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
