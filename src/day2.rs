use itertools::Itertools;

#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<(String, i32)> {
    input
        .split_whitespace()
        .tuples()
        .map(|(d, v)| (d.to_string(), v.parse().unwrap()))
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn part1(input: &[(String, i32)]) -> i32 {
    let position = follow_commands(input);
    position.horizontal * position.aim
}

#[aoc(day2, part2)]
fn part2(input: &[(String, i32)]) -> i32 {
    let position = follow_commands(input);
    position.horizontal * position.depth
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn follow_commands(input: &[(String, i32)]) -> Position {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for (direction, value) in input {
        match (direction.as_str(), value) {
            ("forward", value) => {
                position.horizontal += value;
                position.depth += position.aim * value;
            }
            ("down", value) => position.aim += value,
            ("up", value) => position.aim -= value,
            _ => unreachable!(),
        }
    }
    position
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            [
                ("forward".to_string(), 5),
                ("down".to_string(), 5),
                ("forward".to_string(), 8),
                ("up".to_string(), 3),
                ("down".to_string(), 8),
                ("forward".to_string(), 2)
            ]
        );
    }

    #[test]
    fn day2_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn day2_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 900);
    }
}
