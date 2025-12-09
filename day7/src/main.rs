use bitvec::prelude::*;
use std::collections::HashMap;
use std::time::Instant;
use utils::{Grid, read_input};

// DP state: map beam patterns to number of ways to reach them
type Pattern = BitVec;
type Count = u128;
type Level = HashMap<Pattern, Count>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '^' => Cell::Splitter,
            'S' => Cell::Beam,
            _ => panic!("Unknown cell type: {}", c),
        }
    }
}

fn parse_grid(input: &str) -> Grid<Cell> {
    let data: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect();
    Grid::new(data)
}

fn simulate_row(grid: &mut Grid<Cell>, row: usize) -> i64 {
    let mut changes: Vec<(usize, usize, Cell)> = Vec::new();
    let mut split_count = 0;

    for col in 0..grid.cols() {
        if let Some(&Cell::Beam) = grid.get(row, col) {
            let below_row = row + 1;

            match grid.get(below_row, col) {
                Some(&Cell::Empty) => {
                    changes.push((below_row, col, Cell::Beam));
                }
                Some(&Cell::Splitter) => {
                    let mut split_happened = false;

                    if let Some(&cell_left) = grid.get(below_row, col.saturating_sub(1)) {
                        if col > 0 && cell_left != Cell::Beam {
                            changes.push((below_row, col - 1, Cell::Beam));
                            split_happened = true;
                        }
                    }

                    if let Some(&cell_right) = grid.get(below_row, col + 1) {
                        if cell_right != Cell::Beam {
                            changes.push((below_row, col + 1, Cell::Beam));
                            split_happened = true;
                        }
                    }

                    if split_happened {
                        split_count += 1;
                    }
                }
                Some(&Cell::Beam) | None => {}
            }
        }
    }

    for (r, c, new_cell) in changes {
        if let Some(cell) = grid.get_mut(r, c) {
            *cell = new_cell;
        }
    }

    split_count
}

fn solve_part1(input: &str) -> i64 {
    let mut grid = parse_grid(input);
    let mut total_splits = 0;

    for row in 0..grid.rows() {
        total_splits += simulate_row(&mut grid, row);
    }

    total_splits
}

fn initial_row_beams(grid: &Grid<Cell>) -> Pattern {
    let cols = grid.cols();
    let mut bits = bitvec![0; cols];
    let row = 0;
    for col in 0..cols {
        if let Some(&Cell::Beam) = grid.get(row, col) {
            bits.set(col, true);
        }
    }
    bits
}

fn count_splits_for_row(
    base_grid: &Grid<Cell>,
    row: usize,
    cols: usize,
    row_beams: &Pattern,
) -> usize {
    if row + 1 >= base_grid.rows() {
        return 0;
    }
    let mut splits = 0;
    for col in 0..cols {
        if row_beams[col] {
            if let Some(&Cell::Splitter) = base_grid.get(row + 1, col) {
                splits += 1;
            }
        }
    }
    splits
}

fn advance_row_with_choice(
    base_grid: &Grid<Cell>,
    row: usize,
    cols: usize,
    row_beams: &Pattern,
    choice_mask: usize, // Each bit = left(1) or right(0) for each splitter
) -> Pattern {
    let mut next_row = bitvec![0; cols];
    if row + 1 >= base_grid.rows() {
        return next_row;
    }

    let mut split_idx = 0;
    for col in 0..cols {
        if !row_beams[col] {
            continue;
        }

        match base_grid.get(row + 1, col) {
            Some(&Cell::Splitter) => {
                // Check bit to decide left or right
                let go_left = (choice_mask & (1 << split_idx)) != 0;
                split_idx += 1;

                if go_left && col > 0 {
                    next_row.set(col - 1, true);
                } else if !go_left && col + 1 < cols {
                    next_row.set(col + 1, true);
                }
            }
            Some(&Cell::Empty) => {
                next_row.set(col, true);
            }
            _ => {}
        }
    }
    next_row
}

fn solve_part2(input: &str) -> i64 {
    let base_grid = parse_grid(input);
    let rows = base_grid.rows();
    let cols = base_grid.cols();

    let initial = initial_row_beams(&base_grid);

    let mut current_level: Level = HashMap::new();
    current_level.insert(initial, 1);

    // Process row by row, tracking unique patterns + counts
    for row in 0..rows {
        let mut next_level: Level = HashMap::new();

        for (row_beams, &ways) in current_level.iter() {
            if row_beams.not_any() {
                continue;
            }

            let k = count_splits_for_row(&base_grid, row, cols, row_beams);

            if k == 0 {
                // No splits, deterministic path
                let next_pattern = advance_row_with_choice(&base_grid, row, cols, row_beams, 0);
                *next_level.entry(next_pattern).or_insert(0) += ways;
            } else {
                // k splits means 2^k possible choices
                let combinations = 1usize << k;
                for choice_mask in 0..combinations {
                    let next_pattern =
                        advance_row_with_choice(&base_grid, row, cols, row_beams, choice_mask);
                    // Merge duplicate patterns, accumulate counts
                    *next_level.entry(next_pattern).or_insert(0) += ways;
                }
            }
        }

        current_level = next_level;
        if current_level.is_empty() {
            break;
        }
    }

    current_level.values().copied().sum::<Count>() as i64
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day7.txt");

    let input = read_input("./inputs/day7.txt")?;
    let (part1, part2) = solve(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 40);
    }
}
