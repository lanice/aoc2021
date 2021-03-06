use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[aoc_generator(day12)]
fn generator_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .flat_map(|connection| connection.split('-'))
        .map(|cave| cave.to_string())
        .collect_vec()
        .into_iter()
        .tuples()
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[(String, String)]) -> i32 {
    let (connections, big_caves) = parse_caves(input);
    visit_next(
        "start",
        &connections,
        &big_caves,
        &HashSet::from(["start"]),
        false,
        false,
    )
}

#[aoc(day12, part2)]
fn part2(input: &[(String, String)]) -> i32 {
    let (connections, big_caves) = parse_caves(input);
    visit_next(
        "start",
        &connections,
        &big_caves,
        &HashSet::from(["start"]),
        true,
        false,
    )
}

fn parse_caves(input: &[(String, String)]) -> (HashMap<&str, Vec<&str>>, HashSet<&str>) {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut big_caves: HashSet<&str> = HashSet::new();

    for (left, right) in input {
        for (l, r) in [(left, right), (right, left)] {
            if l != "end" && r != "start" {
                connections.entry(l).or_insert_with(Vec::new).push(r);
            }
        }

        for cave in [left, right] {
            if cave.chars().next().unwrap().is_uppercase() {
                big_caves.insert(cave);
            }
        }
    }

    (connections, big_caves)
}

fn visit_next(
    current_cave: &str,
    connections: &HashMap<&str, Vec<&str>>,
    big_caves: &HashSet<&str>,
    visited: &HashSet<&str>,
    can_visit_small_cave_twice: bool,
    did_visit_small_cave_twice: bool,
) -> i32 {
    if current_cave == "end" {
        return 1;
    }

    let mut successful_paths = 0;

    for cave in &connections[current_cave] {
        let visited_and_small = visited.contains(cave) && !big_caves.contains(cave);
        let can_visit_twice = can_visit_small_cave_twice && !did_visit_small_cave_twice;
        let did_visit_twice = did_visit_small_cave_twice || can_visit_twice && visited_and_small;
        let can_visit = !visited_and_small || can_visit_twice;

        if can_visit {
            let mut visited = visited.clone();
            visited.insert(cave);
            successful_paths += visit_next(
                cave,
                connections,
                big_caves,
                &visited,
                can_visit_small_cave_twice,
                did_visit_twice,
            );
        }
    }

    successful_paths
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    static INPUT2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            vec![
                ("start".to_string(), "A".to_string()),
                ("start".to_string(), "b".to_string()),
                ("A".to_string(), "c".to_string()),
                ("A".to_string(), "b".to_string()),
                ("b".to_string(), "d".to_string()),
                ("A".to_string(), "end".to_string()),
                ("b".to_string(), "end".to_string()),
            ]
        );
    }

    #[test]
    fn day12_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 10);
        let input2 = generator_input(INPUT2);
        assert_eq!(part1(&input2), 19);
    }

    #[test]
    fn day12_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 36);
    }
}
