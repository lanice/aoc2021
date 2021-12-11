use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<Entry> {
    input.lines().map(|a| Entry::from_str(a)).collect_vec()
}

#[aoc(day8, part1)]
fn part1(input: &[Entry]) -> usize {
    input.iter().fold(0, |acc, e| {
        acc + e
            .output
            .iter()
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count()
    })
}

#[aoc(day8, part2)]
fn part2(input: &[Entry]) -> i32 {
    input.iter().map(decode).sum()
}

fn decode(entry: &Entry) -> i32 {
    let histogram = build_histogram(&entry.signal);
    decode_output(&histogram, &entry.output)
}

fn build_histogram(signal: &[String]) -> HashMap<char, i32> {
    signal.iter().fold(HashMap::new(), |acc, digit| {
        digit.chars().fold(acc, |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
    })
}

fn decode_output(histogram: &HashMap<char, i32>, output: &[String]) -> i32 {
    output
        .iter()
        .map(|n| n.chars().map(|ch| histogram[&ch]).sum())
        .map(|digit_id| match digit_id {
            42 => 0,
            17 => 1,
            34 => 2,
            39 => 3,
            30 => 4,
            37 => 5,
            41 => 6,
            25 => 7,
            49 => 8,
            45 => 9,
            _ => unreachable!(),
        })
        .fold(0, |acc, digit| acc * 10 + digit)
}

struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

impl Entry {
    fn from_str(input: &str) -> Self {
        let (signal_raw, output_raw) = input.split_once(" | ").unwrap();
        let signal = signal_raw
            .split_whitespace()
            .map(|s| s.to_string())
            .collect_vec();
        let output = output_raw
            .split_whitespace()
            .map(|s| s.to_string())
            .collect_vec();
        Entry { signal, output }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input.len(), 10);
        assert_eq!(
            input[0].signal,
            vec![
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb"
            ]
        );
        assert_eq!(input[0].output, vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]);
        assert_eq!(
            input[9].signal,
            vec![
                "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                "fegbdc"
            ]
        );
        assert_eq!(input[9].output, vec!["fgae", "cfgab", "fg", "bagce"]);
    }

    #[test]
    fn day8_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn day8_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 61229);
    }
}
