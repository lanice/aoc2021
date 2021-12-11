use std::collections::HashMap;

const BOARD_SIZE: usize = 5;

#[aoc_generator(day4)]
fn generator_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut blocks = input.split("\n\n");
    let numbers: Vec<i32> = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let boards: Vec<Board> = blocks.map(|b| Board::new(b)).collect();

    (numbers, boards)
}

#[aoc(day4, part1)]
fn part1((numbers, boards): &(Vec<i32>, Vec<Board>)) -> i32 {
    let mut boards: Vec<Board> = boards.to_vec();

    for n in numbers {
        for board in boards.iter_mut() {
            board.process_number(n);
            if board.has_won {
                return board.calc_score(n);
            }
        }
    }

    unreachable!();
}

#[aoc(day4, part2)]
fn part2((numbers, boards): &(Vec<i32>, Vec<Board>)) -> i32 {
    let mut boards: Vec<Board> = boards.to_vec();
    let mut last_winning_score = 0;

    for n in numbers {
        for board in boards.iter_mut() {
            if board.has_won {
                continue;
            }

            board.process_number(n);
            if board.has_won {
                last_winning_score = board.calc_score(n);
            }
        }
    }

    last_winning_score
}

#[derive(Debug, PartialEq, Clone)]
struct Board {
    numbers: HashMap<i32, (usize, usize)>,
    rows_filled: Vec<usize>,
    columns_filled: Vec<usize>,
    has_won: bool,
}

impl Board {
    fn new(input: &str) -> Board {
        let mut numbers = HashMap::new();
        let rows: Vec<&str> = input.lines().collect();
        for (i, row_raw) in rows.iter().enumerate() {
            let row: Vec<i32> = row_raw
                .split_whitespace()
                .map(|f| f.parse::<i32>().unwrap())
                .collect();
            for (j, cell) in row.iter().enumerate() {
                numbers.insert(*cell, (i, j));
            }
        }

        Board {
            numbers,
            rows_filled: vec![0; BOARD_SIZE],
            columns_filled: vec![0; BOARD_SIZE],
            has_won: false,
        }
    }

    fn process_number(&mut self, n: &i32) {
        if let Some((i, j)) = self.numbers.remove(n) {
            self.rows_filled[i] += 1;
            self.columns_filled[j] += 1;
        }
        self.has_won = self.rows_filled.contains(&5) || self.columns_filled.contains(&5);
    }

    fn calc_score(&self, last_called: &i32) -> i32 {
        let unmarked_sum: i32 = self.numbers.keys().sum();
        unmarked_sum * last_called
    }
}

#[cfg(test)]
pub mod tests {
    use std::{array::IntoIter, collections::HashMap, vec};

    use crate::day4::BOARD_SIZE;

    use super::{generator_input, part1, part2, Board};

    static INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn generator() {
        let (numbers, boards) = generator_input(INPUT);
        let exp_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let exp_board = Board {
            numbers: HashMap::<_, _>::from_iter(IntoIter::new([
                (22, (0, 0)),
                (13, (0, 1)),
                (17, (0, 2)),
                (11, (0, 3)),
                (0, (0, 4)),
                (8, (1, 0)),
                (2, (1, 1)),
                (23, (1, 2)),
                (4, (1, 3)),
                (24, (1, 4)),
                (21, (2, 0)),
                (9, (2, 1)),
                (14, (2, 2)),
                (16, (2, 3)),
                (7, (2, 4)),
                (6, (3, 0)),
                (10, (3, 1)),
                (3, (3, 2)),
                (18, (3, 3)),
                (5, (3, 4)),
                (1, (4, 0)),
                (12, (4, 1)),
                (20, (4, 2)),
                (15, (4, 3)),
                (19, (4, 4)),
            ])),
            rows_filled: vec![0; BOARD_SIZE],
            columns_filled: vec![0; BOARD_SIZE],
            has_won: false,
        };
        assert_eq!(numbers, exp_numbers);
        assert_eq!(boards[0], exp_board);
    }

    #[test]
    fn day4_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn day4_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 1924);
    }
}
