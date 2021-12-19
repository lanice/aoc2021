#[aoc_generator(day17)]
fn generator_input(input: &str) -> Area {
    let (l, r) = input
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();
    let (x1, x2) = l.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let (y1, y2) = r.strip_prefix("y=").unwrap().split_once("..").unwrap();

    Area {
        tl: Point {
            x: x1.parse().unwrap(),
            y: y2.parse().unwrap(),
        },
        br: Point {
            x: x2.parse().unwrap(),
            y: y1.parse().unwrap(),
        },
    }
}

#[aoc(day17, part1)]
fn part1(area: &Area) -> i32 {
    let (max_y, _) = brute_force_shots(area);
    max_y
}

#[aoc(day17, part2)]
fn part2(area: &Area) -> i32 {
    let (_, vel_count) = brute_force_shots(area);
    vel_count
}

fn brute_force_shots(area: &Area) -> (i32, i32) {
    let mut max_y = 0;
    let mut vel_count = 0;
    for x in -1000..1000 {
        for y in -1000..1000 {
            if let Some(local_max_y) = shoot_probe(area, (x, y)) {
                max_y = max_y.max(local_max_y);
                vel_count += 1;
            }
        }
    }
    (max_y, vel_count)
}

fn shoot_probe(area: &Area, (vel_x, vel_y): (i32, i32)) -> Option<i32> {
    let mut pos = Point { x: 0, y: 0 };
    let mut vel = Point { x: vel_x, y: vel_y };
    let mut max_y = 0;

    while !area.is_inside(&pos) && (pos.y > area.br.y - 1000) {
        max_y = max_y.max(pos.y);
        pos.x += vel.x;
        pos.y += vel.y;
        vel.x += if vel.x < 0 {
            1
        } else if vel.x > 0 {
            -1
        } else {
            0
        };
        vel.y -= 1;
    }

    if area.is_inside(&pos) {
        return Some(max_y);
    }

    None
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Area {
    tl: Point,
    br: Point,
}

impl Area {
    fn is_inside(&self, point: &Point) -> bool {
        self.tl.x <= point.x && point.x <= self.br.x && self.br.y <= point.y && point.y <= self.tl.y
    }
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, Point};

    static INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input.tl, Point { x: 20, y: -5 });
        assert_eq!(input.br, Point { x: 30, y: -10 });
    }

    #[test]
    #[ignore] // expensive brute-force :)
    fn day17_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 45);
    }

    #[test]
    #[ignore] // expensive brute-force :)
    fn day17_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 112);
    }
}
