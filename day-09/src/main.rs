fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(str::split_whitespace)
        .map(|i| i.map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn predict(series: &[i32]) -> (i32, i32) {
    let mut turtles = vec![series.to_owned()];
    loop {
        let bottom = turtles.last().unwrap();
        let new_bottom: Vec<_> = bottom
            .iter()
            .zip(bottom.iter().skip(1))
            .map(|(i, j)| j - i)
            .collect();
        if new_bottom.is_empty() || new_bottom.iter().all(|x| *x == 0) {
            break;
        }
        turtles.push(new_bottom);
    }

    let future = turtles.iter().map(|t| t.last().unwrap()).sum();
    let past = turtles
        .iter()
        .zip([1, -1].iter().cycle())
        .map(|(t, sign)| sign * t.first().unwrap())
        .sum();
    (past, future)
}

fn part1(input: &str) -> i32 {
    parse(input).iter().map(|s| predict(s.as_ref()).1).sum()
}

fn part2(input: &str) -> i32 {
    parse(input).iter().map(|s| predict(s.as_ref()).0).sum()
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
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2);
    }
}
