/// Read input file as a string
pub fn read_input(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Read input file and split into lines
pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let content = read_input(path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// Parse a string into a vector of numbers
pub fn parse_numbers<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

/// Parse a range string like "3-5" or "10-100" into a vector of all numbers in the range (inclusive)
pub fn parse_range<T>(s: &str) -> Option<Vec<T>>
where
    T: std::str::FromStr + std::ops::Add<Output = T> + std::cmp::PartialOrd + Copy + From<u8>,
{
    let (start, end) = s.trim().split_once('-')?;
    let start_num = start.trim().parse::<T>().ok()?;
    let end_num = end.trim().parse::<T>().ok()?;

    if start_num > end_num {
        return Some(Vec::new());
    }

    let mut result = Vec::new();
    let mut current = start_num;
    let one = T::from(1);
    while current <= end_num {
        result.push(current);
        current = current + one;
    }
    Some(result)
}

/// Parse a range string like "3-5" into a rust range :D
pub fn parse_range_bounds<T: std::str::FromStr>(s: &str) -> Option<std::ops::RangeInclusive<T>> {
    let (start, end) = s.trim().split_once('-')?;
    let start_num = start.trim().parse::<T>().ok()?;
    let end_num = end.trim().parse::<T>().ok()?;
    Some(start_num..=end_num)
}

/// Transpose a rectangular matrix of rows into columns.
///
/// Input shape:  rows: &[Vec<T>]   (N rows, M columns)
/// Output shape: Vec<Vec<T>>       (M rows, N columns)
///
/// Example:
///     [ [1,2,3],
///       [4,5,6] ]  becomes
///     [ [1,4],
///       [2,5],
///       [3,6] ]
///
/// Works for any `T: Clone`. All input rows must have the same length.
pub fn transpose_matrix<T: Clone>(rows: &[Vec<T>]) -> Vec<Vec<T>> {
    if rows.is_empty() {
        return Vec::new();
    }

    let cols = rows[0].len();

    (0..cols)
        .map(|c| rows.iter().map(|row| row[c].clone()).collect())
        .collect()
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.get(0).map(|row| row.len()).unwrap_or(0)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Get coordinates of neighbors (4-directional: up, down, left, right)
    pub fn neighbor_coords_4(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let rows = self.rows();
        let cols = self.cols();

        if row > 0 {
            result.push((row - 1, col));
        }
        if row + 1 < rows {
            result.push((row + 1, col));
        }
        if col > 0 {
            result.push((row, col - 1));
        }
        if col + 1 < cols {
            result.push((row, col + 1));
        }
        result
    }

    /// Get neighbors with their values (4-directional: up, down, left, right)
    /// Returns Vec<(row, col, &value)>
    pub fn neighbors_4(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        self.neighbor_coords_4(row, col)
            .into_iter()
            .filter_map(|(r, c)| self.get(r, c).map(|val| (r, c, val)))
            .collect()
    }

    /// Get coordinates of neighbors (8-directional: including diagonals)
    pub fn neighbor_coords_8(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let rows = self.rows() as i32;
        let cols = self.cols() as i32;

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
                    result.push((new_row as usize, new_col as usize));
                }
            }
        }
        result
    }

    /// Get neighbors with their values (8-directional: including diagonals)
    /// Returns Vec<(row, col, &value)>
    pub fn neighbors_8(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        self.neighbor_coords_8(row, col)
            .into_iter()
            .filter_map(|(r, c)| self.get(r, c).map(|val| (r, c, val)))
            .collect()
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, cell)| (row_idx, col_idx, cell))
        })
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: std::fmt::Display> Grid<T> {
    pub fn pretty_print(&self) {
        for row in &self.data {
            for cell in row {
                print!("{cell}");
            }
            println!();
        }
    }
}

impl<T: Clone> Grid<T> {
    /// Create a grid of `rows` × `cols`, filled with `value`.
    pub fn filled(rows: usize, cols: usize, value: T) -> Self {
        let data = vec![vec![value; cols]; rows];
        Self { data }
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn parse_char_grid(input: &str) -> Grid<char> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(data)
}

use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}

impl Rational {
    pub fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "denominator cannot be zero");
        let mut n = num;
        let mut d = den;
        if d < 0 {
            n = -n;
            d = -d;
        }
        let g = gcd(n, d);
        Rational {
            num: n / g,
            den: d / g,
        }
    }

    pub fn from_i64(n: i64) -> Self {
        Rational { num: n, den: 1 }
    }

    pub fn zero() -> Self {
        Rational { num: 0, den: 1 }
    }

    pub fn one() -> Self {
        Rational { num: 1, den: 1 }
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // handle negatives
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    if a == 0 { 1 } else { a }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

// a + b
impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        Rational::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

// a - b
impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Rational {
        Rational::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

// a * b
impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        Rational::new(self.num * rhs.num, self.den * rhs.den)
    }
}

// a / b
impl Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Rational {
        assert!(rhs.num != 0, "division by zero rational");
        Rational::new(self.num * rhs.den, self.den * rhs.num)
    }
}

pub fn rref(mut mat: Vec<Vec<Rational>>) -> (Vec<Vec<Rational>>, Vec<Option<usize>>) {
    let rows = mat.len();
    if rows == 0 {
        return (mat, Vec::new());
    }
    let cols = mat[0].len();
    for r in &mat {
        assert_eq!(
            r.len(),
            cols,
            "all rows must have the same number of columns"
        );
    }

    let mut pivot_row = 0usize;
    let mut pivot_cols: Vec<Option<usize>> = vec![None; rows];

    for col in 0..cols {
        if pivot_row >= rows {
            break;
        }

        let mut sel: Option<usize> = None;
        for r in pivot_row..rows {
            if !mat[r][col].is_zero() {
                sel = Some(r);
                break;
            }
        }

        let row = match sel {
            Some(r) => r,
            None => continue,
        };

        if row != pivot_row {
            mat.swap(row, pivot_row);
        }

        let pivot_val = mat[pivot_row][col];
        for c in col..cols {
            mat[pivot_row][c] = mat[pivot_row][c] / pivot_val;
        }

        for r in 0..rows {
            if r == pivot_row {
                continue;
            }
            let factor = mat[r][col];
            if factor.is_zero() {
                continue;
            }
            for c in col..cols {
                mat[r][c] = mat[r][c] - factor * mat[pivot_row][c];
            }
        }

        pivot_cols[pivot_row] = Some(col);
        pivot_row += 1;
    }

    (mat, pivot_cols)
}
