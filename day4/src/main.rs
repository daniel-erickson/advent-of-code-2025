use std::time::Instant;
use utils::{parse_char_grid, read_input};

fn solve_part1(input: &str) -> i64 {
    let grid = parse_char_grid(input);
    grid.iter_cells()
        .filter(|&(row, col, value)| {
            let neighbours = grid.neighbors_8(row, col);
            match value {
                '@' => neighbours.iter().filter(|&(_, _, v)| **v == '@').count() < 4,
                '.' => false,
                _ => false,
            }
        })
        .count() as i64
}

fn solve_part2(input: &str) -> i64 {
    let mut grid = parse_char_grid(input);
    let mut count = 0;

    loop {
        let maybe_item = grid.iter_cells().find(|&(row, col, value)| {
            let neighbours = grid.neighbors_8(row, col);
            match value {
                '@' => neighbours.iter().filter(|&(_, _, v)| **v == '@').count() < 4,
                '.' => false,
                _ => false,
            }
        });

        match maybe_item {
            Some((row, col, _)) => {
                grid[row][col] = '.';
                count += 1;
            }
            None => break,
        }
    }

    count
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day4.txt");

    let input = read_input("./inputs/day4.txt")?;
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

    const TEST_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 13); // Update expected value
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 0); // Update expected value
    }
}
