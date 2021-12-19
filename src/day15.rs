use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

#[aoc_generator(day15)]
fn generator_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|a| {
            a.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day15, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    dijkstra(input)
}

#[aoc(day15, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let (orig_h, orig_w) = (input.len(), input[0].len());
    let (h, w) = (orig_h * 5, orig_w * 5);
    let mut grid = vec![vec![0; w]; h];
    for h_tile in 0..5 {
        for w_tile in 0..5 {
            for i in 0..orig_h {
                for j in 0..orig_w {
                    let mut digit = input[i][j] + h_tile + w_tile;
                    while digit > 9 {
                        digit -= 9;
                    }
                    grid[h_tile as usize * orig_h + i][w_tile as usize * orig_w + j] = digit;
                }
            }
        }
    }
    dijkstra(&grid)
}

fn dijkstra(input: &[Vec<i32>]) -> i32 {
    let (h, w) = (input.len() as i32, input[0].len() as i32);
    let source = (0, 0);
    let destination = (h - 1, w - 1);

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, source)));

    let mut min_dist = HashMap::from([(source, 0)]);
    let mut visited = HashSet::new();

    while !heap.is_empty() {
        let (dist, node) = heap.pop().unwrap().0;

        if node == destination {
            return dist;
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for neighbor in neighbors(node.0, node.1, h, w) {
            if visited.contains(&neighbor) {
                continue;
            }

            let new_dist = dist + input[neighbor.0 as usize][neighbor.1 as usize];
            if new_dist < *min_dist.get(&neighbor).unwrap_or(&i32::MAX) {
                min_dist.insert(neighbor, new_dist);
                heap.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    i32::MAX
}

fn neighbors(row: i32, col: i32, h: i32, w: i32) -> Vec<(i32, i32)> {
    let mut neighbors = vec![];
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (x, y) = (row + dx, col + dy);
        if 0 <= x && x < h && 0 <= y && y < w {
            neighbors.push((x, y));
        }
    }
    neighbors
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(
            input,
            vec![
                vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
                vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
                vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
                vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
                vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
                vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
                vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
                vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
                vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
                vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
            ]
        );
    }

    #[test]
    fn day15_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn day15_part2() {
        let input = generator_input(INPUT);
        assert_eq!(part2(&input), 315);
    }
}
