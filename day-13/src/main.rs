use std::{collections::HashSet, usize};

type Matrix = Vec<Vec<bool>>;
type EncodedMatrix = (Vec<u64>, Vec<u64>);

fn parse(input: &str) -> Matrix {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

#[inline]
fn encode(matrix: &Matrix) -> EncodedMatrix {
    fn to_bits(seq: &[bool]) -> u64 {
        seq.iter().fold(0, |acc, b| (acc << 1) + *b as u64)
    }
    let rows: Vec<_> = matrix.iter().map(|r| to_bits(r.as_slice())).collect();

    let cols: Vec<_> = (0..matrix[0].len())
        .map(|j| matrix.iter().map(|r| r[j]).collect())
        .map(|c: Vec<_>| to_bits(c.as_slice()))
        .collect();
    (rows, cols)
}

fn find_mirrors(seq: &[u64]) -> HashSet<usize> {
    let mut mirrors = HashSet::new();
    for i in 1..(seq.len()) {
        let side_len = i.min(seq.len() - i);
        if seq[i - side_len..i]
            .iter()
            .eq(seq[i..i + side_len].iter().rev())
        {
            mirrors.insert(i);
        }
    }
    mirrors
}

fn score(input: &str, smudge: bool) -> usize {
    let mut matrix = parse(input);
    let (rows, cols) = encode(&matrix);

    let mut row_mirrors = find_mirrors(rows.as_slice());
    let mut col_mirrors = find_mirrors(cols.as_slice());
    if smudge {
        let orig_row_mirrors = row_mirrors.clone();
        let orig_col_mirrors = col_mirrors.clone();
        'smudging: for i in 0..rows.len() {
            for j in 0..cols.len() {
                matrix[i][j] = !matrix[i][j];
                let (rows, cols) = encode(&matrix);
                
                row_mirrors = &find_mirrors(rows.as_slice()) - &orig_row_mirrors;
                col_mirrors = &find_mirrors(cols.as_slice()) - &orig_col_mirrors;

                if !row_mirrors.is_empty() ||  !col_mirrors.is_empty(){
                    break 'smudging;
                }
                matrix[i][j] = !matrix[i][j];
            }
        }
    }

    row_mirrors.iter().map(|x| 100 * x).sum::<usize>() + col_mirrors.iter().sum::<usize>()
}

fn part1(input: &str) -> usize {
    input.split("\n\n").map(|s| score(s, false)).sum()
}

fn part2(input: &str) -> usize {
    input.split("\n\n").map(|s| score(s, true)).sum()
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
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 400);
    }
}
