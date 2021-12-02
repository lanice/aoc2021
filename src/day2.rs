#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|instr| Instruction::new(instr))
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let position = follow_commands(input);
    position.horizontal * position.aim
}

#[aoc(day2, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let position = follow_commands(input);
    position.horizontal * position.depth
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl Direction {
    pub fn new(direction: &str) -> Direction {
        match direction {
            "forward" => Direction::FORWARD,
            "down" => Direction::DOWN,
            "up" => Direction::UP,
            _ => unreachable!(),
        }
    }
}

struct Instruction {
    direction: Direction,
    value: i32,
}

impl Instruction {
    pub fn new(instr: &str) -> Instruction {
        let split = instr.split_whitespace().collect::<Vec<_>>();
        let direction = Direction::new(split[0]);
        let value = split[1].parse::<i32>().unwrap();
        Instruction { direction, value }
    }
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn follow_commands(input: &[Instruction]) -> Position {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for instruction in input {
        match instruction.direction {
            Direction::FORWARD => {
                position.horizontal += instruction.value;
                position.depth += position.aim * instruction.value;
            }
            Direction::DOWN => position.aim += instruction.value,
            Direction::UP => position.aim -= instruction.value,
        }
    }
    position
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
