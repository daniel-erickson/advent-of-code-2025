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

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn parse_char_grid(input: &str) -> Grid<char> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(data)
}
