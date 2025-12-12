use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use utils::{Grid, read_input};

fn solve_part1(input: &str) -> i64 {
    let points: Vec<[i64; 2]> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            [x, y]
        })
        .collect();

    let mut max_volume = 0_i64;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let [x1, y1] = points[i];
            let [x2, y2] = points[j];
            if x1 != x2 && y1 != y2 {
                let x_length = (x1 - x2).abs() + 1;
                let y_length = (y1 - y2).abs() + 1;
                let volume = x_length * y_length;

                if (volume > max_volume) {
                    max_volume = volume;
                }
            }
        }
    }

    max_volume
}

fn rectangle_is_inside(
    grid: &Grid<char>,
    row_min: usize,
    row_max: usize,
    col_min: usize,
    col_max: usize,
) -> bool {
    for r in row_min..=row_max {
        for c in col_min..=col_max {
            if let Some(ch) = grid.get(r, c) {
                if *ch == '.' {
                    return false;
                }
            }
        }
    }
    true
}

fn solve_part2(input: &str) -> i64 {
    // Parse points in world coords
    let points: Vec<[i64; 2]> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            [x, y]
        })
        .collect();

    // collect unique x's and y's
    let mut xs: Vec<i64> = points.iter().map(|p| p[0]).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p[1]).collect();

    xs.sort();
    xs.dedup();

    ys.sort();
    ys.dedup();

    // Maps from world coord to compressed index
    let x_index: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_index: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    // Build compressed points
    let compressed_points: Vec<[usize; 2]> = points
        .iter()
        .map(|&[x, y]| {
            let cx = x_index[&x];
            let cy = y_index[&y];
            [cy, cx]
        })
        .collect();

    // Compressed grid size
    let width = xs.len();
    let height = ys.len();

    let mut grid = Grid::filled(height, width, '.');

    // Set the original points as '#'
    for &[row, col] in &compressed_points {
        if let Some(cell) = grid.get_mut(row, col) {
            *cell = '#';
        }
    }
    // Connect each point to the next with 'X'
    for i in 0..compressed_points.len() {
        let [r1, c1] = compressed_points[i];
        let [r2, c2] = compressed_points[(i + 1) % compressed_points.len()];

        if c1 == c2 {
            // same column make a vertical segment
            let col = c1;
            let r_min = r1.min(r2);
            let r_max = r1.max(r2);

            for r in r_min..=r_max {
                if let Some(cell) = grid.get_mut(r, col) {
                    if *cell != '#' {
                        *cell = 'X';
                    }
                }
            }
        } else if r1 == r2 {
            // same row make a horizontal segment
            let row = r1;
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);

            for c in c_min..=c_max {
                if let Some(cell) = grid.get_mut(row, c) {
                    if *cell != '#' {
                        *cell = 'X';
                    }
                }
            }
        }
    }

    // Flood fill from outside on the compressed grid
    let rows = grid.rows();
    let cols = grid.cols();

    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();

    // push a border cell into the queue if it's '.' and not yet visited
    let mut push = |r: usize, c: usize| {
        if !visited[r][c] {
            if matches!(grid.get(r, c), Some('.')) {
                visited[r][c] = true;
                queue.push_back((r, c));
            }
        }
    };

    // finds all the dots on the edges of the grid and queue them as BFS seed
    for c in 0..cols {
        push(0, c);
        push(rows - 1, c);
    }
    for r in 0..rows {
        push(r, 0);
        push(r, cols - 1);
    }

    // Breadth-First Search over '.' region from the outside
    while let Some((r, c)) = queue.pop_front() {
        for (nr, nc) in grid.neighbor_coords_4(r, c) {
            if !visited[nr][nc] && matches!(grid.get(nr, nc), Some('.')) {
                visited[nr][nc] = true;
                queue.push_back((nr, nc));
            }
        }
    }

    // Any '.' that was not visited is enclosed fill with 'X'
    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                if let Some(ch) = grid.get_mut(r, c) {
                    if *ch == '.' {
                        *ch = 'X';
                    }
                }
            }
        }
    }

    let mut best_area: i64 = 0;

    for i in 0..compressed_points.len() {
        for j in (i + 1)..compressed_points.len() {
            let [r1, c1] = compressed_points[i];
            let [r2, c2] = compressed_points[j];

            if r1 == r2 || c1 == c2 {
                continue;
            }

            let row_min = r1.min(r2);
            let row_max = r1.max(r2);
            let col_min = c1.min(c2);
            let col_max = c1.max(c2);

            // check if this rectangle is fully inside
            if rectangle_is_inside(&grid, row_min, row_max, col_min, col_max) {
                // convert back to real world.
                let world_x_min = xs[col_min];
                let world_x_max = xs[col_max];
                let world_y_min = ys[row_min];
                let world_y_max = ys[row_max];

                let width = (world_x_max - world_x_min).abs() + 1;
                let height = (world_y_max - world_y_min).abs() + 1;
                let area = width * height;

                if area > best_area {
                    best_area = area;
                }
            }
        }
    }

    // Nice debug print :D
    // grid.pretty_print();

    best_area
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day9.txt");

    let input = read_input("./inputs/day9.txt")?;
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

    const TEST_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 24);
    }
}
