use std::{collections::HashMap, vec};

#[aoc_generator(day10)]
fn generator_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Vec<char>]) -> i32 {
    let map = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let score_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut score = 0;

    for line in input {
        let mut stack = vec![];
        for &char in line {
            match char {
                '(' | '{' | '[' | '<' => stack.push(char),
                c => {
                    if stack.pop().unwrap() != map[&c] {
                        score += score_map[&c];
                        break;
                    }
                }
            }
        }
    }

    score
}

#[aoc(day10, part2)]
fn part2(input: &[Vec<char>]) -> u64 {
    let map = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let score_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores = vec![];

    for line in input {
        let mut stack = vec![];
        let mut score = 0u64;
        for &char in line {
            match char {
                '(' | '{' | '[' | '<' => stack.push(char),
                c => {
                    if stack.pop().unwrap() != map[&c] {
                        stack.clear();
                        break;
                    }
                }
            }
        }
        while let Some(c) = stack.pop() {
            score *= 5;
            score += score_map[&c];
        }
        if score > 0 {
            scores.push(score);
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input[0],
            vec![
                '[', '(', '{', '(', '<', '(', '(', ')', ')', '[', ']', '>', '[', '[', '{', '[',
                ']', '{', '<', '(', ')', '<', '>', '>'
            ]
        );
        assert_eq!(
            input[9],
            vec![
                '<', '{', '(', '[', '{', '{', '}', '}', '[', '<', '[', '[', '[', '<', '>', '{',
                '}', ']', ']', ']', '>', '[', ']', ']'
            ]
        );
    }

    #[test]
    fn day10_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn day10_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 288957);
    }
}
