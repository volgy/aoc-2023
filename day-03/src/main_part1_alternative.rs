use core::num;


#[derive(Debug)]
struct Schematic {
    width: usize,
    height: usize,
    sheet: Vec<Vec<char>>,
}

impl Schematic {
    fn parse(input: &str) -> Self {
        let mut sheet = Vec::new();
        let width = input.lines().next().unwrap().chars().count();
        for line in input.lines() {
            let row: Vec<_> = line.chars().collect();
            assert_eq!(row.len(), width);
            sheet.push(row);
        }
        let width = sheet.iter().next().unwrap().len();
        let height = sheet.len();
        Self {
            width,
            height,
            sheet,
        }
    }
}

fn touches_symbol(schematic: &Schematic, row: usize, col: usize) -> bool {
    let start_row = (row as isize - 1).max(0);
    let start_col = (col as isize - 1).max(0);
    let end_row = (row as isize + 1).min(schematic.height as isize - 1);
    let end_col = (col as isize + 1).min(schematic.width as isize - 1);

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            let c = schematic.sheet[i as usize][j as usize];
            if !c.is_ascii_digit() && c != '.' {
                return true;
            }
        }
    }
    false
}

fn part1(input: &str) -> u32 {
    let schematic = Schematic::parse(input);
    let mut sum = 0;

    let mut num_str = String::new();
    let mut is_partno = false;

    fn finalize(num_str: &mut String, is_partno: &mut bool) -> u32 {
        let mut result = 0;
        if !num_str.is_empty() && *is_partno {
            result = num_str.parse::<u32>().unwrap();
        }
        num_str.clear();
        *is_partno = false;
        result
    }

    for (row, line) in schematic.sheet.iter().enumerate() {
        let line = line.iter().enumerate();
        for (col, ch) in line {
            if ch.is_ascii_digit() {
                is_partno = is_partno || touches_symbol(&schematic, row, col);
                num_str.push(*ch);
            } else {
                sum += finalize(&mut num_str, &mut is_partno);
            }
        }
        sum += finalize(&mut num_str, &mut is_partno);
    }
    sum
}

fn part2(input: &str) -> u32 {
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

    #[test]
    fn test_part1() {
        let input = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = "\
        \n\
        ";

        assert_eq!(part2(input), 0);
    }
}
