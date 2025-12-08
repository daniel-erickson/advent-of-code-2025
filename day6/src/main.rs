use std::time::Instant;
use utils::{read_input, transpose_matrix};

fn solve_part1(input: &str) -> i64 {
    let chars: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let transposed = transpose_matrix(&chars);

    let totals: Vec<i64> = transposed
        .iter()
        .map(|f| {
            let op = *f.last().unwrap();
            let values = &f[..f.len() - 1];

            match op {
                "+" => values.iter().map(|s| s.parse::<i64>().unwrap()).sum(),
                "*" => values.iter().map(|s| s.parse::<i64>().unwrap()).product(),
                _ => panic!("unexpected operator"),
            }
        })
        .collect();

    totals.iter().sum()
}

fn parse_grid(input: &str) -> Vec<Vec<Vec<String>>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let max_width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let operator_row_idx = lines.len() - 1;
    let operator_line = lines[operator_row_idx];
    let operator_chars: Vec<char> = operator_line.chars().collect();

    let mut operator_positions = Vec::new();
    for (idx, ch) in operator_chars.iter().enumerate() {
        if !ch.is_whitespace() {
            operator_positions.push(idx);
        }
    }

    let mut is_separator = vec![false; max_width];
    for pair in operator_positions.windows(2) {
        let right = pair[1];
        if right > 0 {
            let sep_col = right - 1;
            let ch_at_sep = operator_chars.get(sep_col).copied().unwrap_or(' ');
            if ch_at_sep == ' ' {
                is_separator[sep_col] = true;
            }
        }
    }

    let mut result: Vec<Vec<Vec<String>>> = Vec::with_capacity(lines.len());

    for (row_idx, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut row_groups: Vec<Vec<String>> = Vec::new();
        let mut current_group: Vec<String> = Vec::new();

        for col in 0..max_width {
            if is_separator[col] {
                if !current_group.is_empty() {
                    row_groups.push(current_group);
                    current_group = Vec::new();
                }
                continue;
            }

            let raw_ch = chars.get(col).copied().unwrap_or(' ');

            if row_idx == operator_row_idx {
                if !raw_ch.is_whitespace() {
                    current_group.push(raw_ch.to_string());
                }
            } else {
                let ch = if raw_ch == ' ' { '0' } else { raw_ch };
                current_group.push(ch.to_string());
            }
        }

        if !current_group.is_empty() {
            row_groups.push(current_group);
        }

        result.push(row_groups);
    }

    result
}

fn row_to_value(digits: &[String]) -> i64 {
    let mut end = digits.len();
    while end > 1 && digits[end - 1] == "0" {
        end -= 1;
    }

    let s = digits[..end].join("");
    if s.is_empty() {
        0
    } else {
        s.parse::<i64>().unwrap()
    }
}

fn solve_part2(input: &str) -> i64 {
    let grid = parse_grid(input);

    let transposed = transpose_matrix(&grid);

    let re_transposed: Vec<Vec<Vec<String>>> = transposed
        .iter()
        .map(|block| {
            let (digits, op_slice) = block.split_at(block.len() - 1);
            let op_row = op_slice[0].clone(); // ["*"] or ["+"]

            let mut out = transpose_matrix(digits);
            out.push(op_row);

            out
        })
        .collect();

    let totals: Vec<i64> = re_transposed
        .iter()
        .map(|block| {
            let (digit_rows, op_slice) = block.split_at(block.len() - 1);
            let op = op_slice[0][0].as_str();

            let values: Vec<i64> = digit_rows
                .iter()
                .rev()
                .map(|row| row_to_value(row))
                .collect();

            match op {
                "+" => values.iter().sum(),
                "*" => values.iter().product(),
                _ => panic!("unexpected operator {op}"),
            }
        })
        .collect();

    totals.iter().sum()
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day6.txt");

    let input = read_input("./inputs/day6.txt")?;
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

    const TEST_INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 3263827);
    }
}
