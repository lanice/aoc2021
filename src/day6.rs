#[aoc_generator(day6)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[i32]) -> usize {
    populate(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[i32]) -> usize {
    populate(input, 256)
}

fn populate(input: &[i32], days: usize) -> usize {
    let mut age_counts = vec![0usize;9];

    for &f in input {
        age_counts[f as usize] += 1;
    }

    for _ in 0..days {
        age_counts.rotate_left(1);
        age_counts[6] += age_counts[8];
    }

    age_counts.iter().sum::<usize>()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn day6_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 5934);
    }

    #[test]
    fn day6_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 26984457539);
    }
}
