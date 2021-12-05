use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|a| {
            a.split(" -> ")
                .map(|p| Point::from_str(p))
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[(Point, Point)]) -> usize {
    let mut map: Map = HashMap::new();

    for (p1, p2) in input {
        if p1.x != p2.x && p1.y != p2.y {
            continue;
        }
        traverse_vent(&mut map, (p1, p2));
    }

    map.values().filter(|&&v| v >= 2).count()
}

#[aoc(day5, part2)]
fn part2(input: &[(Point, Point)]) -> usize {
    let mut map: Map = HashMap::new();

    for (p1, p2) in input {
        traverse_vent(&mut map, (p1, p2));
    }

    map.values().filter(|&&v| v >= 2).count()
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(str: &str) -> Point {
        let (x, y) = str
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        Point { x, y }
    }
}

type Map = HashMap<(i32, i32), i32>;

fn traverse_vent(map: &mut Map, (p1, p2): (&Point, &Point)) {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let direction_x = get_direction(dx);
    let direction_y = get_direction(dy);
    let distance = dx.abs().max(dy.abs());

    for i in 0..=distance {
        let point = (p1.x + i * direction_x, p1.y + i * direction_y);
        *map.entry(point).or_insert(0) += 1;
    }
}

fn get_direction(distance: i32) -> i32 {
    if distance > 0 {
        -1
    } else if distance < 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, Point};

    static INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input[0], (Point { x: 0, y: 9 }, Point { x: 5, y: 9 }));
        assert_eq!(input[1], (Point { x: 8, y: 0 }, Point { x: 0, y: 8 }));
    }

    #[test]
    fn day5_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn day5_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 12);
    }
}
