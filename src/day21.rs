use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day21)]
fn generator_input(input: &str) -> (u32, u32) {
    input
        .lines()
        .map(|a| a.chars().last().unwrap().to_digit(10).unwrap())
        .collect_tuple()
        .unwrap()
}

#[aoc(day21, part1)]
fn part1(input: &(u32, u32)) -> u64 {
    let mut die = DeterministicDie::new();
    let (mut p1_pos, mut p2_pos) = (input.0 as u64, input.1 as u64);
    let (mut p1_score, mut p2_score) = (0, 0);

    while p2_score < 1000 {
        p1_pos = move_pawn(p1_pos, die.roll_3());
        p1_score += p1_pos;

        if p1_score >= 1000 {
            return p2_score * die.total_rolls;
        }

        p2_pos = move_pawn(p2_pos, die.roll_3());
        p2_score += p2_pos;
    }

    p1_score * die.total_rolls
}

#[aoc(day21, part2)]
fn part2(input: &(u32, u32)) -> u64 {
    let quantum_rolls = get_quantum_rolls();
    let (p1_wins, p2_wins) = game_turn(
        input.0 as u64,
        0,
        input.1 as u64,
        0,
        Player::P1,
        &quantum_rolls,
        &mut HashMap::new(),
    );
    p1_wins.max(p2_wins)
}

fn move_pawn(pos: u64, steps: u64) -> u64 {
    (pos + steps - 1) % 10 + 1
}

fn game_turn(
    p1_pos: u64,
    p1_score: u64,
    p2_pos: u64,
    p2_score: u64,
    on_play: Player,
    quantum_rolls: &[u64],
    memo: &mut HashMap<(u64, u64, u64, u64, Player), (u64, u64)>,
) -> (u64, u64) {
    if p1_score >= 21 {
        return (1, 0);
    }
    if p2_score >= 21 {
        return (0, 1);
    }
    if let Some(&result) = memo.get(&(p1_pos, p1_score, p2_pos, p2_score, on_play)) {
        return result;
    }

    let (mut p1_wins, mut p2_wins) = (0, 0);

    for &roll in quantum_rolls {
        let (new_p1_pos, new_p1_score, new_p2_pos, new_p2_score, next_on_play) = match on_play {
            Player::P1 => {
                let pos = move_pawn(p1_pos, roll);
                (pos, p1_score + pos, p2_pos, p2_score, Player::P2)
            }
            Player::P2 => {
                let pos = move_pawn(p2_pos, roll);
                (p1_pos, p1_score, pos, p2_score + pos, Player::P1)
            }
        };
        let (p1_new_wins, p2_new_wins) = game_turn(
            new_p1_pos,
            new_p1_score,
            new_p2_pos,
            new_p2_score,
            next_on_play,
            quantum_rolls,
            memo,
        );
        p1_wins += p1_new_wins;
        p2_wins += p2_new_wins;
    }

    memo.insert(
        (p1_pos, p1_score, p2_pos, p2_score, on_play),
        (p1_wins, p2_wins),
    );

    (p1_wins, p2_wins)
}

fn get_quantum_rolls() -> Vec<u64> {
    let mut quantum_rolls = vec![];

    for d1 in 1..4 {
        for d2 in 1..4 {
            for d3 in 1..4 {
                quantum_rolls.push(d1 + d2 + d3);
            }
        }
    }

    quantum_rolls
}

struct DeterministicDie {
    last_rolled: u64,
    total_rolls: u64,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie {
            last_rolled: 0,
            total_rolls: 0,
        }
    }

    fn roll(&mut self) -> u64 {
        self.total_rolls += 1;
        self.last_rolled = (self.last_rolled) % 100 + 1;
        self.last_rolled
    }

    fn roll_3(&mut self) -> u64 {
        (0..3).fold(0, |acc, _| acc + self.roll())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Player {
    P1,
    P2,
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, move_pawn, part1, part2, DeterministicDie};

    static INPUT: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input, (4, 8));
    }

    #[test]
    fn day21_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 739785);
    }

    #[test]
    fn day21_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 444356092776315);
    }

    #[test]
    fn day21_move_pawn() {
        assert_eq!(move_pawn(1, 5), 6);
        assert_eq!(move_pawn(1, 10), 1);
        assert_eq!(move_pawn(1, 9), 10);
        assert_eq!(move_pawn(10, 0), 10);
        assert_eq!(move_pawn(10, 33), 3);
    }

    #[test]
    fn day21_die() {
        let mut die = DeterministicDie::new();
        assert_eq!(die.roll(), 1);
        assert_eq!(die.roll(), 2);
        assert_eq!(die.roll(), 3);

        assert_eq!(die.roll_3(), 15);
        assert_eq!(die.roll_3(), 24);
        assert_eq!(die.roll_3(), 33);
    }
}
