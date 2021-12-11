use std::collections::HashSet;

use itertools::Itertools;

#[aoc_generator(day9)]
fn generator_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|a| {
            a.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    let mut lows = vec![];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_low(input, i, j) {
                lows.push(input[i][j]);
            }
        }
    }

    lows.iter().sum::<i32>() + lows.len() as i32
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let mut lows_coords = vec![];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_low(input, i, j) {
                lows_coords.push((i, j));
            }
        }
    }

    let mut lows_sizes = lows_coords
        .iter()
        .map(|&(i, j)| {
            1 + flood_neighbors(
                input,
                &mut HashSet::from([(i as i32, j as i32)]),
                i as i32,
                j as i32,
            )
        })
        .collect_vec();

    lows_sizes.sort();
    lows_sizes.reverse();

    lows_sizes[0..3].iter().product()
}

fn is_low(input: &[Vec<i32>], i: usize, j: usize) -> bool {
    let i_start = if i == 0 { i } else { i - 1 };
    let j_start = if j == 0 { j } else { j - 1 };
    let i_end = if i == input.len() - 1 { i } else { i + 1 };
    let j_end = if j == input[i].len() - 1 { j } else { j + 1 };

    for i_neighbor in i_start..=i_end {
        for j_neighbor in j_start..=j_end {
            if i_neighbor == i && j_neighbor == j {
                continue;
            }

            if input[i][j] >= input[i_neighbor][j_neighbor] {
                return false;
            }
        }
    }

    true
}

fn flood_neighbors(
    input: &[Vec<i32>],
    flooded_coords: &mut HashSet<(i32, i32)>,
    i: i32,
    j: i32,
) -> i32 {
    let height = input[i as usize][j as usize];
    let mut size = 0;
    let neighbors = vec![(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)];

    for (n_i, n_j) in neighbors {
        if n_i < 0
            || n_i >= input.len() as i32
            || n_j < 0
            || n_j >= input[i as usize].len() as i32
            || flooded_coords.contains(&(n_i, n_j))
        {
            continue;
        }

        let n_height = input[n_i as usize][n_j as usize];
        if n_height > height && n_height != 9 {
            flooded_coords.insert((n_i, n_j));
            size += flood_neighbors(input, flooded_coords, n_i, n_j);
            size += 1;
        }
    }

    size
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ]
        );
    }

    #[test]
    fn day9_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn day9_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 1134);
    }
}
