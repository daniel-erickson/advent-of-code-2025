use std::{ops::RangeInclusive, time::Instant};
use utils::{parse_range_bounds, read_input};

/// Split ranges and ingredients
pub fn split_range_ingredients(input: &str) -> (&str, &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    match parts.as_slice() {
        [a, b] => (*a, *b),
        _ => panic!("expected exactly two sections separated by a blank line"),
    }
}

/// Split the line into two parts, ranges and items
/// Iterate through ranges and then look through ingredients
/// discard any ingredient in this range and update a counter of fresh.
fn solve_part1(input: &str) -> i64 {
    let (ranges_str, ingredients_str) = split_range_ingredients(input);

    let ranges: Vec<RangeInclusive<i64>> = ranges_str
        .lines()
        .filter_map(parse_range_bounds::<i64>)
        .collect();

    let mut ingredients: Vec<i64> = ingredients_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut fresh_count = 0;

    ingredients.retain(|ingredient| {
        if ranges.iter().any(|range| range.contains(ingredient)) {
            fresh_count += 1;
            false
        } else {
            true
        }
    });

    fresh_count
}

/// Will sort the ranges by start value.
/// Walk through them and fold in or extend the range.
/// If we hit a gap then we start a new range.
fn solve_part2(input: &str) -> i64 {
    let (ranges_str, _) = split_range_ingredients(input);

    let mut ranges: Vec<RangeInclusive<i64>> = ranges_str
        .lines()
        .filter_map(parse_range_bounds::<i64>)
        .collect();

    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<i64>> = Vec::new();

    for r in ranges {
        let start = *r.start();
        let end = *r.end();

        if let Some(last) = merged.last_mut() {
            let last_start = *last.start();
            let last_end = *last.end();

            if start <= last_end + 1 {
                if end > last_end {
                    *last = last_start..=end;
                }
            } else {
                merged.push(start..=end);
            }
        } else {
            merged.push(start..=end);
        }
    }

    merged
        .iter()
        .map(|r| *r.end() - *r.start() + 1)
        .sum::<i64>()
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day5.txt");

    let input = read_input("./inputs/day5.txt")?;
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

    const TEST_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 14);
    }
}
