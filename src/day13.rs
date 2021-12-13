use std::collections::HashSet;

use itertools::Itertools;

#[aoc_generator(day13)]
fn generator_input(input: &str) -> (Paper, Vec<(Axis, i32)>) {
    let (dots_raw, instructions_raw) = input.split_once("\n\n").unwrap();
    let dots = HashSet::from_iter(dots_raw.lines().map(|l| {
        l.split(',')
            .map(|number| number.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    }));
    let (mut width, mut height) = (0, 0);
    for (x, y) in &dots {
        width = width.max(*x);
        height = height.max(*y);
    }
    let paper = Paper {
        dots,
        width: width + 1,
        height: height + 1,
    };
    let instructions = instructions_raw
        .lines()
        .map(|l| {
            let (axis, coord) = l
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            (Axis::new(axis), coord.parse::<i32>().unwrap())
        })
        .collect_vec();

    (paper, instructions)
}

#[aoc(day13, part1)]
fn part1((paper, instructions): &(Paper, Vec<(Axis, i32)>)) -> usize {
    let mut paper = paper.clone();
    let (axis, coord) = &instructions[0];
    paper.fold(axis, *coord);

    paper.dots.len()
}

#[aoc(day13, part2)]
fn part2((paper, instructions): &(Paper, Vec<(Axis, i32)>)) -> usize {
    let mut paper = paper.clone();

    for (axis, coord) in instructions {
        paper.fold(axis, *coord);
    }

    // This solution is presented by: The human eye :)
    paper.print();

    0 // return type other than () needed because of cargo-aoc
}

#[derive(Debug, PartialEq)]
enum Axis {
    X,
    Y,
}

impl Axis {
    fn new(s: &str) -> Self {
        match s {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Paper {
    dots: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Paper {
    fn fold(&mut self, axis: &Axis, coord: i32) {
        let dots = self.dots.drain().collect_vec();
        for (x, y) in dots {
            match axis {
                Axis::X => {
                    let new_x = if x < coord { x } else { x - 2 * (x - coord) };
                    self.dots.insert((new_x, y));
                }
                Axis::Y => {
                    let new_y = if y < coord { y } else { y - 2 * (y - coord) };
                    self.dots.insert((x, new_y));
                }
            }
        }
        match axis {
            Axis::X => self.width -= coord + 1,
            Axis::Y => self.height -= coord + 1,
        }
    }

    fn print(self) {
        for y in 0..self.height + 1 {
            let mut row = vec![];
            for x in 0..self.width + 1 {
                if self.dots.contains(&(x, y)) {
                    row.push("█");
                } else {
                    row.push("░");
                }
            }
            println!("{}", row.join(""));
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, Axis};

    static INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn generator() {
        let (paper, instructions) = generator_input(INPUT);
        assert!(paper.dots.contains(&(6, 10)));
        assert!(paper.dots.contains(&(0, 14)));
        assert!(paper.dots.contains(&(8, 10)));
        assert!(paper.dots.contains(&(9, 0)));
        assert!(!paper.dots.contains(&(0, 0)));
        assert!(!paper.dots.contains(&(9, 9)));
        assert_eq!(paper.width, 11);
        assert_eq!(paper.height, 15);
        assert_eq!(instructions, vec![(Axis::Y, 7), (Axis::X, 5)]);
    }

    #[test]
    fn day13_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 17);
    }
}
