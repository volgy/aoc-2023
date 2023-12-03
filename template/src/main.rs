fn part1(input: &str) -> u32 {
    todo!();
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
        \n\
        ";

        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        
        let input = "\
        \n\
        ";

        assert_eq!(part2(input), 0);
    }
}
