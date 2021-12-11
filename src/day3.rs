#[aoc_generator(day3)]
fn generator_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|a| a.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u32>]) -> i32 {
    let len = input[0].len();
    let mut sums: Vec<u32> = vec![0; len];

    for column in input {
        for j in 0..len {
            sums[j] += column[j];
        }
    }

    let mut gamma: Vec<u32> = vec![0; len];
    let mut epsil: Vec<u32> = vec![1; len];

    for i in 0..sums.len() {
        if sums[i] * 2 > input.len() as u32 {
            gamma[i] = 1;
            epsil[i] = 0;
        }
    }

    to_decimal(&gamma) * to_decimal(&epsil)
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u32>]) -> i32 {
    let oxy = &calc(input, 0);
    let co2 = &calc(input, 1);
    to_decimal(oxy) * to_decimal(co2)
}

fn to_decimal(binary: &[u32]) -> i32 {
    binary
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &elem)| acc + elem as i32 * 2_i32.pow(i as u32))
}

fn calc(report: &[Vec<u32>], xor_bit: u32) -> Vec<u32> {
    let mut report = report.to_vec();

    for i in 0..report[0].len() {
        let common_bit = most_common_at_position(&report, i) ^ xor_bit;
        report.retain(|bits| bits[i] == common_bit);
        if report.len() == 1 {
            return report.remove(0);
        }
    }
    unreachable!();
}

fn most_common_at_position(report: &[Vec<u32>], i: usize) -> u32 {
    (report.iter().filter(|line| line[i] == 1).count() * 2 >= report.len()) as u32
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            vec![
                vec![0, 0, 1, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 1, 1, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![0, 1, 1, 1, 1],
                vec![0, 0, 1, 1, 1],
                vec![1, 1, 1, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
            ]
        );
    }

    #[test]
    fn day3_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn day3_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 230);
    }
}
