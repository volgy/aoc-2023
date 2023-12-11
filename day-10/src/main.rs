use std::collections::HashMap;
use std::iter;

type Pos = (isize, isize);

struct Maze(HashMap<Pos, char>);

impl Maze {
    fn parse(input: &str) -> Maze {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(row, line)| iter::repeat(row).enumerate().zip(line.chars()))
                .map(|((col, row), c)| ((col as isize, row as isize), c))
                .collect(),
        )
    }

    fn start(&self) -> Pos {
        self.0
            .iter()
            .find_map(|(&pos, c)| match c {
                'S' => Some(pos),
                _ => None,
            })
            .unwrap()
    }

    fn longest_path(&self) -> (Vec<Pos>, char) {
        let start = self.start();
        let mut paths = Vec::new();

        for start_dir in "NESW".chars() {
            let mut dir = start_dir;
            let mut pos = start;
            let mut path = vec![pos];
            loop {
                let north_pos = (pos.0, pos.1 - 1);
                let east_pos = (pos.0 + 1, pos.1);
                let south_pos = (pos.0, pos.1 + 1);
                let west_pos = (pos.0 - 1, pos.1);

                let north = self.0.get(&north_pos);
                let east = self.0.get(&east_pos);
                let south = self.0.get(&south_pos);
                let west = self.0.get(&west_pos);

                // little hack: dir contains the actual start symbol at the end
                (dir, pos) = match (dir, north, east, south, west) {
                    ('N', Some('|'), ..) => ('N', north_pos),
                    ('N', Some('F'), ..) => ('E', north_pos),
                    ('N', Some('7'), ..) => ('W', north_pos),
                    ('N', Some('S'), ..) => (
                        match start_dir {
                            'E' => 'F',
                            'N' => '|',
                            'W' => '7',
                            _ => panic!(),
                        },
                        north_pos,
                    ),
                    ('E', _, Some('-'), ..) => ('E', east_pos),
                    ('E', _, Some('J'), ..) => ('N', east_pos),
                    ('E', _, Some('7'), ..) => ('S', east_pos),
                    ('E', _, Some('S'), ..) => (
                        match start_dir {
                            'N' => 'J',
                            'E' => '-',
                            'S' => '7',
                            _ => panic!(),
                        },
                        east_pos,
                    ),
                    ('S', .., Some('|'), _) => ('S', south_pos),
                    ('S', .., Some('L'), _) => ('E', south_pos),
                    ('S', .., Some('J'), _) => ('W', south_pos),
                    ('S', .., Some('S'), _) => (
                        match start_dir {
                            'E' => 'L',
                            'S' => '|',
                            'W' => 'J',
                            _ => panic!(),
                        },
                        south_pos,
                    ),
                    ('W', .., Some('-')) => ('W', west_pos),
                    ('W', .., Some('L')) => ('N', west_pos),
                    ('W', .., Some('F')) => ('S', west_pos),
                    ('W', .., Some('S')) => (
                        match start_dir {
                            'N' => 'L',
                            'W' => '-',
                            'S' => 'F',
                            _ => panic!(),
                        },
                        west_pos,
                    ),
                    _ => break,
                };

                path.push(pos);

                if self.0[&pos] == 'S' {
                    paths.push((path, dir));
                    break;
                }
            }
        }
        paths
            .into_iter()
            .reduce(|a, b| if a.0.len() >= b.0.len() { a } else { b })
            .unwrap()
    }

    fn enclosed_area(&self) -> usize {
        let (path, start_symbol) = self.longest_path();

        let (max_x, max_y) = self.0.keys().max().unwrap();

        let mut area = 0;
        for y in 0..=*max_y {
            let mut inside = false;
            let mut prev_elbow = None;
            for x in 0..=*max_x {
                let pos = (x, y);
                let symbol = self.0[&pos];
                let symbol = if symbol == 'S' { start_symbol } else { symbol };
                if path.contains(&pos) {
                    if symbol == '|' {
                        inside = !inside;
                    } else if "FL".contains(symbol) {
                        prev_elbow = Some(symbol);
                    } else if (symbol == '7' && prev_elbow.unwrap() == 'L')
                        || (symbol == 'J' && prev_elbow.unwrap() == 'F')
                    {
                        inside = !inside;
                    }
                } else if inside {
                    area += 1;
                }
            }
        }
        area
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    maze.longest_path().0.len() / 2
}

fn part2(input: &str) -> usize {
    let maze = Maze::parse(input);
    maze.enclosed_area()
}

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", part1(text));
    println!("Part 2: {}", part2(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
            .....\n\
            .S-7.\n\
            .|.|.\n\
            .L-J.\n\
            .....";
        assert_eq!(part1(input), 4);

        let input = "\
            ..F7.\n\
            .FJ|.\n\
            SJ.L7\n\
            |F--J\n\
            LJ...";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = "\
            ...........\n\
            .S-------7.\n\
            .|F-----7|.\n\
            .||.....||.\n\
            .||.....||.\n\
            .|L-7.F-J|.\n\
            .|..|.|..|.\n\
            .L--J.L--J.\n\
            ...........";
        assert_eq!(part2(input), 4);

        let input = "\
            FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input), 10);
    }
}
