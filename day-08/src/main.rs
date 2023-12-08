use std::collections::HashMap;

fn parse(input: &str, use_wildcards: bool) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("invalid instruction"),
        })
        .collect();

    let graph: HashMap<_, _> = lines
        .skip(1)
        .map(|l| l.split_once(" = ").unwrap())
        .map(|(n, p)| (n, p.trim_matches(&['(', ')'][..]).split_once(", ").unwrap()))
        .collect();

    let mut packed_graph = vec![usize::MAX; 2 * graph.len()];
    let lut: HashMap<_, _> = graph.keys().zip(0..).map(|(k, v)| (k, 2 * v)).collect();
    for (node, (left, right)) in &graph {
        packed_graph[lut[node]] = lut[left];
        packed_graph[lut[node] + 1] = lut[right];
    }

    let endpoints = |name: &str| {
        if use_wildcards {
            let pattern = name.chars().last().unwrap();
            lut.iter()
                .filter_map(|(label, idx)| {
                    if label.ends_with(pattern) {
                        Some(*idx)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![lut[&name]]
        }
    };

    let starts = endpoints("AAA");
    let ends = endpoints("ZZZ");

    (instructions, packed_graph, starts, ends)
}

fn lcm(values: &[usize]) -> usize {
    if values.len() == 1 {
        return values[0];
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    let a = values[0];
    let b = lcm(&values[1..]);
    a * b / gcd(a, b)
}

fn part1(input: &str) -> usize {
    let (instructions, graph, starts, ends) = parse(input, false);
    let end = ends[0];
    let mut steps = 0;
    let mut current = starts[0];
    for &next in instructions.iter().cycle() {
        if current == end {
            break;
        }
        current = graph[current + next];
        steps += 1;
    }
    steps
}

fn part2(input: &str) -> usize {
    let (instructions, graph, starts, ends) = parse(input, true);

    let mut periods = Vec::new();
    for mut current in starts.into_iter() {
        let mut steps: usize = 0;
        for &next in instructions.iter().cycle() {
            if ends.contains(&current) {
                break;
            }
            current = graph[current + next];
            steps += 1;
        }
        periods.push(steps)
    }
    lcm(&periods[..])
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
            RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 2);

        let input = "\
            LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
