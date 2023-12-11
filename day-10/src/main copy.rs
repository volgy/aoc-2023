use std::collections::{HashMap, HashSet};
use std::iter;

type Pos = (isize, isize);

struct Maze {
    map: HashMap<Pos, char>,
    start: Pos,
}

impl Maze {
    fn parse(input: &str) -> Maze {
        let map: HashMap<_, _> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| iter::repeat(row).enumerate().zip(line.chars()))
            .map(|((col, row), c)| ((col as isize, row as isize), c))
            .collect();

        let start = map
            .iter()
            .find_map(|(&pos, c)| match c {
                'S' => Some(pos),
                _ => None,
            })
            .unwrap();

        Self { map, start }
    }

    fn longest_path(&self, pos: &Pos, mut visited: HashSet<Pos>) -> Option<usize> {
        visited.insert(*pos);
        println!("{pos:?}");
        fn ports(symbol: char) -> &'static str {
            match symbol {
                '|' => "NS",
                '-' => "EW",
                'L' => "NE",
                'J' => "NW",
                '7' => "SW",
                'F' => "SE",
                'S' => "NESW",
                _ => "",
            }
        }

        let mut max_steps = None;

        for (exit, entry, next_pos) in [
            ('N', 'S', (pos.0, pos.1 - 1)),
            ('E', 'W', (pos.0 + 1, pos.1)),
            ('S', 'N', (pos.0, pos.1 + 1)),
            ('W', 'E', (pos.0 - 1, pos.1)),
        ] {
            if let Some(&neighbor) = self.map.get(&next_pos) {
                if neighbor == 'S' && visited.len() > 1 {
                    max_steps = Some(max_steps.unwrap_or(1));
                }
                if visited.contains(&next_pos) {
                    continue;
                }
                if ports(self.map[pos]).contains(exit) && ports(neighbor).contains(entry) {
                    if let Some(remaining_steps) = self.longest_path(&next_pos, visited.clone()) {
                        max_steps = Some(max_steps.unwrap_or(0).max(remaining_steps + 1));
                    }
                }
            }
        }
        max_steps
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    maze.longest_path(&maze.start, HashSet::new()).unwrap() / 2
}

fn part2(input: &str) -> usize {
    todo!();
}

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", part1(text));
    println!("Part 2: {}", part2(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....";

    const INPUT2: &str = "\
        ..F7.\n\
        .FJ|.\n\
        SJ.L7\n\
        |F--J\n\
        LJ...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 4);
        assert_eq!(part1(INPUT2), 8);
    }

    #[test]
    fn test_part2() {
        //assert_eq!(part2(INPUT), 0);
    }
}
