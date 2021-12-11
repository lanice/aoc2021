use itertools::Itertools;

#[aoc_generator(day7)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    let desired_pos = input.iter().sorted_unstable().nth(input.len() / 2).unwrap();
    calculate_fuel_p1(input, *desired_pos)
}

#[aoc(day7, part2)]
fn part2(input: &[i32]) -> i32 {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    (min..=max).fold(i32::MAX, |acc, desired_pos| {
        acc.min(calculate_fuel_p2(input, desired_pos))
    })
}

fn calculate_fuel_p1(input: &[i32], desired_pos: i32) -> i32 {
    input
        .iter()
        .fold(0, |acc, pos| acc + (pos - desired_pos).abs())
}

fn calculate_fuel_p2(input: &[i32], desired_pos: i32) -> i32 {
    input.iter().fold(0, |acc, pos| {
        acc + nth_triangular((pos - desired_pos).abs())
    })
}

fn nth_triangular(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn day7_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn day7_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 168);
    }
}
