use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day14)]
fn generator_input(input: &str) -> (String, HashMap<String, char>) {
    let (polymer, rules_raw) = input.split_once("\n\n").unwrap();

    let rules = HashMap::from_iter(rules_raw.lines().map(|l| {
        let (l, r) = l.split_once(" -> ").unwrap();
        (l.to_string(), r.parse().unwrap())
    }));

    (polymer.to_string(), rules)
}

#[aoc(day14, part1)]
fn part1((polymer, rules): &(String, HashMap<String, char>)) -> u64 {
    find_formula(polymer, rules, 10)
}

#[aoc(day14, part2)]
fn part2((polymer, rules): &(String, HashMap<String, char>)) -> u64 {
    find_formula(polymer, rules, 40)
}

fn find_formula(polymer: &String, rules: &HashMap<String, char>, steps: i32) -> u64 {
    let mut counts: HashMap<char, u64> = HashMap::new();
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();

    for elem in polymer.chars() {
        *counts.entry(elem).or_insert(0) += 1;
    }

    for pair in polymer.chars().tuple_windows() {
        *pairs.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..steps {
        step(rules, &mut counts, &mut pairs);
    }

    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();

    most - least
}

fn step(
    rules: &HashMap<String, char>,
    counts: &mut HashMap<char, u64>,
    pairs: &mut HashMap<(char, char), u64>,
) {
    let prev_pairs = pairs.drain().collect_vec();
    for ((left, right), count) in prev_pairs {
        let middle = rules[&format!("{}{}", left, right)];
        *counts.entry(middle).or_insert(0) += count;
        *pairs.entry((left, middle)).or_insert(0) += count;
        *pairs.entry((middle, right)).or_insert(0) += count;
    }
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn generator() {
        let (polymer, rules) = generator_input(INPUT);
        assert_eq!(polymer, "NNCB");
        assert_eq!(rules["CH"], 'B');
        assert_eq!(rules["HH"], 'N');
        assert_eq!(rules["CN"], 'C');
    }

    #[test]
    fn day14_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn day14_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 2188189693529);
    }
}
