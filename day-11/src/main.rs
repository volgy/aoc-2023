use std::collections::HashSet;
use std::iter;

use itertools::Itertools;

type Pos = (usize, usize);

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| iter::repeat(row).enumerate().zip(line.chars()))
        .filter_map(|((col, row), c)| if c == '#' { Some((col, row)) } else { None })
        .collect()
}

fn expand(galaxies: &mut [Pos], factor: usize) {
    let (mut min_x, mut max_x) = (usize::MAX, 0);
    let (mut min_y, mut max_y) = (usize::MAX, 0);

    let mut galaxy_xs = HashSet::new();
    let mut galaxy_ys = HashSet::new();
    for &(x, y) in galaxies.iter() {
        galaxy_xs.insert(x);
        galaxy_ys.insert(y);
        (min_x, max_x) = (min_x.min(x), max_x.max(x));
        (min_y, max_y) = (min_y.min(x), max_y.max(x));
    }

    let all_xs: HashSet<_> = (min_x..=max_x).collect();
    let all_ys: HashSet<_> = (min_y..=max_y).collect();

    let empty_cols = &all_xs - &galaxy_xs;
    let empty_rows = &all_ys - &galaxy_ys;

    for (x, y) in galaxies {
        *x += (factor - 1) * empty_cols.iter().filter(|c| *c < x).count();
        *y += (factor - 1) * empty_rows.iter().filter(|r| *r < y).count();
    }
}

fn distance_sum(galaxies: &[Pos]) -> usize {
    let mut distance_sum = 0;
    for pair in galaxies.iter().combinations(2) {
        distance_sum += (pair[0].0 as isize - pair[1].0 as isize).abs()
            + (pair[0].1 as isize - pair[1].1 as isize).abs();
    }
    distance_sum as usize
}

fn part12(input: &str, factor: usize) -> usize {
    let mut galaxies = parse(input);
    expand(&mut galaxies, factor);
    distance_sum(&galaxies)
}

fn main() {
    let text = include_str!("../inputs/input.txt");
    println!("Part 1: {}", part12(text, 2));
    println!("Part 2: {}", part12(text, 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part12(INPUT, 2), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part12(INPUT, 10), 1030);
        assert_eq!(part12(INPUT, 100), 8410);
    }
}
