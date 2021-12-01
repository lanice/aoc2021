#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    input
        .windows(4)
        .filter(|depths| depths[0] < depths[3])
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }

    #[test]
    fn day1_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn day1_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 5);
    }
}
