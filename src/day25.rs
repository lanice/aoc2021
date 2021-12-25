use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day25)]
fn generator_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect()
}

#[aoc(day25, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let (w, h) = (input[0].len(), input.len());

    let mut souths = HashSet::new();
    let mut easts = HashSet::new();

    for i in 0..h {
        for j in 0..w {
            match input[i][j] {
                'v' => {
                    souths.insert((i, j));
                }
                '>' => {
                    easts.insert((i, j));
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }

    let mut steps = 0;

    loop {
        let (new_souths, new_easts) = step(&souths, &easts, w, h);
        steps += 1;
        if new_souths == souths && new_easts == easts {
            break;
        }
        souths = new_souths;
        easts = new_easts;
    }

    steps
}

#[aoc(day25, part2)]
fn part2(_input: &[Vec<char>]) -> i32 {
    0
}

fn step(
    souths: &HashSet<(usize, usize)>,
    easts: &HashSet<(usize, usize)>,
    w: usize,
    h: usize,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut new_souths = HashSet::new();
    let mut new_easts = HashSet::new();

    for cucumber in easts.iter() {
        let next_pos = fix_pos(&(cucumber.0, cucumber.1 + 1), w, h);
        if souths.contains(&next_pos) || easts.contains(&next_pos) {
            new_easts.insert(*cucumber);
        } else {
            new_easts.insert(next_pos);
        }
    }

    for cucumber in souths.iter() {
        let next_pos = fix_pos(&(cucumber.0 + 1, cucumber.1), w, h);
        if souths.contains(&next_pos) || new_easts.contains(&next_pos) {
            new_souths.insert(*cucumber);
        } else {
            new_souths.insert(next_pos);
        }
    }

    (new_souths, new_easts)
}

fn fix_pos((y, x): &(usize, usize), w: usize, h: usize) -> (usize, usize) {
    let new_y = if *y == h { 0 } else { *y };
    let new_x = if *x == w { 0 } else { *x };
    (new_y, new_x)
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input.len(), 9);
        assert_eq!(input[0].len(), 10);
        assert_eq!(input[0].iter().collect::<String>(), "v...>>.vv>");
        assert_eq!(input[8].iter().collect::<String>(), "....v..v.>");
    }

    #[test]
    fn day25_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 58);
    }

    #[test]
    fn day25_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 0);
    }
}
