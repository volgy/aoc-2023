use std::{
    cmp::{max, Ordering},
    collections::HashMap,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd for Balls {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            Some(Ordering::Greater)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", part1(text));
    println!("Part 2: {}", part2(text));
}

fn part1(input: &str) -> u32 {
    let limits = Balls {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut score = 0;
    for (game_id, min_balls) in min_balls_per_game(input).iter() {
        if limits >= *min_balls {
            score += game_id;
        }
    }
    score
}

fn part2(input: &str) -> u32 {
    let mut score = 0;
    for min_balls in min_balls_per_game(input).values() {
        score += min_balls.red * min_balls.green * min_balls.blue;
    }
    score
}

/// Return the minimum number of balls per game
fn min_balls_per_game(input: &str) -> HashMap<u32, Balls> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let (game_id, game) = line
            .strip_prefix("Game ")
            .unwrap()
            .split_once(": ")
            .unwrap();
        let game_id: u32 = game_id.parse().unwrap();
        let mut min_balls = Balls::default();
        for draw_str in game.split("; ") {
            let mut draw = Balls::default();
            for ball_str in draw_str.split(", ") {
                let (count, color) = ball_str.split_once(" ").unwrap();
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" => draw.red = count,
                    "green" => draw.green = count,
                    "blue" => draw.blue = count,
                    _ => panic!("invalid color"),
                }
            }
            min_balls.red = max(min_balls.red, draw.red);
            min_balls.green = max(min_balls.green, draw.green);
            min_balls.blue = max(min_balls.blue, draw.blue);
        }

        result.insert(game_id, min_balls);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
