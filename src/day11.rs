use std::collections::{HashSet, VecDeque};

#[aoc_generator(day11)]
fn generator_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|a| a.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<u32>]) -> usize {
    let mut input = input.to_vec();
    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += step(&mut input);
    }

    flash_count
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<u32>]) -> usize {
    let octo_count = input.len() * input[0].len();
    let mut input = input.to_vec();
    let mut step_count = 1;

    while step(&mut input) != octo_count {
        step_count += 1;
    }

    step_count
}

fn step(input: &mut [Vec<u32>]) -> usize {
    let mut who_flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut new_flashes: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            input[i][j] += 1;
            if input[i][j] > 9 {
                who_flashed.insert((i, j));
                new_flashes.push_back((i, j));
            }
        }
    }

    while new_flashes.len() > 0 {
        let (i, j) = new_flashes.pop_front().unwrap();
        for n_i in i.checked_sub(1).unwrap_or(i)..=i + 1 {
            if n_i >= input.len() {
                continue;
            }

            for n_j in j.checked_sub(1).unwrap_or(j)..=j + 1 {
                if n_j >= input[n_i].len() || (n_i, n_j) == (i, j) {
                    continue;
                }

                input[n_i][n_j] += 1;
                if input[n_i][n_j] > 9 && !who_flashed.contains(&(n_i, n_j)) {
                    who_flashed.insert((n_i, n_j));
                    new_flashes.push_back((n_i, n_j));
                }
            }
        }
    }

    for &(i, j) in who_flashed.iter() {
        input[i][j] = 0;
    }

    who_flashed.len()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            vec![
                vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
            ]
        );
    }

    #[test]
    fn day11_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn day11_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 195);
    }
}
