#[aoc_generator(day6)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut ages = input.to_vec();

    for _ in 0..80 {
        let mut new_ages = vec![];
        for &age in &ages {
            if age == 0 {
                new_ages.push(6);
                new_ages.push(8);
            } else {
                new_ages.push(age - 1);
            }
        }
        ages = new_ages;
    }

    ages.len() as i32
}

#[aoc(day6, part2)]
fn part2(input: &[i32]) -> usize {
    let mut age_counts = vec![0usize;9];

    for &age in input {
        age_counts[age as usize] += 1;
    }

    for _ in 0..256 {
        let first = age_counts.drain(0..1).collect::<Vec<_>>()[0];
        age_counts[6] += first;
        age_counts.push(first);
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
