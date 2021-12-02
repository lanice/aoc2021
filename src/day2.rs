#[derive(Debug, Clone, PartialEq)]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl Direction {
    pub fn new(instr: &str) -> Direction {
        match instr {
            "forward" => Direction::FORWARD,
            "down" => Direction::DOWN,
            "up" => Direction::UP,
            _ => panic!("Invalid instruction."),
        }
    }
}

struct Instruction {
    direction: Direction,
    value: i32,
}

fn parse_instruction(line: &str) -> Instruction {
    let split = line.split_whitespace().collect::<Vec<_>>();
    let direction = Direction::new(split[0]);
    let value = split[1].parse::<i32>().unwrap();
    Instruction { direction, value }
}

#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| parse_instruction(l))
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for instruction in input {
        match instruction.direction {
            Direction::FORWARD => horizontal += instruction.value,
            Direction::DOWN => depth += instruction.value,
            Direction::UP => depth -= instruction.value,
        }
    }
    horizontal * depth
}

#[aoc(day2, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instruction in input {
        match instruction.direction {
            Direction::FORWARD => {
                horizontal += instruction.value;
                depth += aim * instruction.value;
            }
            Direction::DOWN => aim += instruction.value,
            Direction::UP => aim -= instruction.value,
        }
    }
    horizontal * depth
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, Direction};

    static INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn generator() {
        let input = generator_input(&INPUT);
        assert_eq!(input[0].direction, Direction::FORWARD);
        assert_eq!(input[0].value, 5);
        assert_eq!(input[1].direction, Direction::DOWN);
        assert_eq!(input[1].value, 5);
        assert_eq!(input[2].direction, Direction::FORWARD);
        assert_eq!(input[2].value, 8);
        assert_eq!(input[5].direction, Direction::FORWARD);
        assert_eq!(input[5].value, 2);
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
