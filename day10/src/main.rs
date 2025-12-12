use std::{collections::VecDeque, time::Instant};
use utils::{read_input, rref, Rational};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Indicator {
    Off,
    On,
}

type Indicators = Vec<Indicator>;

impl Indicator {
    pub fn toggle(self) -> Self {
        match self {
            Indicator::Off => Indicator::On,
            Indicator::On => Indicator::Off,
        }
    }
}

type Sequence = Vec<usize>;

#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<Indicator>,
    buttons: Vec<Sequence>,
    ignition: Vec<Indicator>,
}

impl Machine {
    pub fn min_moves_to_ignition(&mut self) -> i64 {
        if self.indicators == self.ignition {
            return 0;
        }

        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_back(0);

        let mut found = false;

        let mut current_queue: VecDeque<Indicators> = VecDeque::new();
        current_queue.push_back(self.indicators.clone());

        let mut count = 0;

        while !found {
            let mut new_targets: Vec<Indicators> = Vec::new();
            while let Some(state) = current_queue.pop_front() {
                for sequence in &self.buttons {
                    let mut new_indicators = state.clone();
                    for &idx in sequence {
                        if let Some(indicator) = new_indicators.get_mut(idx) {
                            *indicator = indicator.toggle();
                        }
                    }

                    if new_indicators == self.ignition {
                        found = true;
                        break;
                    }
                    new_targets.push(new_indicators.clone());
                }
            }
            count += 1;

            if found {
                break;
            }
            for target in new_targets {
                current_queue.push_back(target);
            }
        }

        count
    }
}

fn parse_machine(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // ignition from first part: ".#.#" -> [Off, On, Off, On]
    let ignition: Vec<Indicator> = parts
        .first()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '.' => Some(Indicator::Off),
            '#' => Some(Indicator::On),
            _ => None,
        })
        .collect();

    // buttons: from 2nd to 2nd-last part, digits only
    let buttons: Vec<Sequence> = parts[1..parts.len() - 1]
        .iter()
        .map(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .collect()
        })
        .collect();

    // indicators start all Off, same length as ignition
    let indicators = vec![Indicator::Off; ignition.len()];

    Machine {
        indicators,
        buttons,
        ignition,
    }
}

fn solve_part1(input: &str) -> i64 {
    let machines: Vec<Machine> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_machine)
        .collect();

    let answer: i64 = machines
        .iter()
        .map(|machine| machine.clone().min_moves_to_ignition())
        .sum();

    answer
}

#[derive(Debug, Clone)]
struct Machine2 {
    target: Vec<i32>,
    buttons: Vec<Vec<usize>>,
}

/// Note for anyyone reading, I got some serious help from chatgpt to implement the solver.
impl Machine2 {
    /// Build the linear system A * x = b for this machine.
    ///
    /// - A is d x m where d = counters, m = buttons
    /// - A[i][j] = 1 if button j affects counter i, else 0
    /// - b[i] = target[i]
    pub fn build_linear_system(&self) -> (Vec<Vec<Rational>>, Vec<Rational>) {
        let d = self.target.len(); // number of counters (equations)
        let m = self.buttons.len(); // number of buttons (variables)

        // Build A: d rows, m columns
        let mut a = vec![vec![Rational::zero(); m]; d];

        for (j, btn) in self.buttons.iter().enumerate() {
            for &counter_idx in btn {
                // for each counter touched by button j, set A[counter][j] = 1
                a[counter_idx][j] = Rational::one();
            }
        }

        // Build b: just target as rationals
        let b = self
            .target
            .iter()
            .map(|&t| Rational::from_i64(t as i64))
            .collect();

        (a, b)
    }

    pub fn rref_augmented(&self) -> (Vec<Vec<Rational>>, Vec<Option<usize>>) {
        let (a, b) = self.build_linear_system();
        let rows = a.len();
        if rows == 0 {
            return (Vec::new(), Vec::new());
        }
        let cols = a[0].len();

        // Build augmented matrix [A | b]
        let mut mat = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols + 1);
            row.extend_from_slice(&a[i]);
            row.push(b[i]);
            mat.push(row);
        }

        rref(mat)
    }
}

impl Machine2 {
    pub fn print_linear_system(&self) {
        let (a, b) = self.build_linear_system();
        let d = a.len(); // counters / equations
        let m = if d > 0 { a[0].len() } else { 0 }; // buttons / variables

        println!("Linear system A * x = b");
        println!("Counters (d) = {}", d);
        println!("Buttons  (m) = {}", m);
        println!();

        for i in 0..d {
            let mut terms = Vec::new();

            for j in 0..m {
                if !a[i][j].is_zero() {
                    terms.push(format!("x{}", j));
                }
            }

            if terms.is_empty() {
                // No button affects this counter.
                println!(
                    "Eq {:02}: 0 = {:?}",
                    i,
                    b[i], // Rational implements Debug
                );
            } else {
                println!("Eq {:02}: {} = {:?}", i, terms.join(" + "), b[i],);
            }
        }

        println!();
    }
}

fn parse_machine2(line: &str) -> Machine2 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(
        parts.len() >= 3,
        "expected at least diagram, one button, target"
    );

    let target_str = parts.last().unwrap();
    let target_inner = target_str
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}');
    let target: Vec<i32> = target_inner
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("invalid target number"))
        .collect();

    let mut buttons = Vec::new();
    for token in &parts[1..parts.len() - 1] {
        let inner = token.trim().trim_start_matches('(').trim_end_matches(')');
        if inner.is_empty() {
            continue;
        }
        let indices: Vec<usize> = inner
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("invalid button index"))
            .collect();
        buttons.push(indices);
    }

    Machine2 { target, buttons }
}

impl Machine2 {
    pub fn min_presses_from_rref(
        &self,
        rref_mat: &[Vec<Rational>],
        pivot_cols: &[Option<usize>],
    ) -> Option<i64> {
        // 1. Determine how many variables we have (buttons)
        let m = self.buttons.len();
        let rows = rref_mat.len();
        if rows == 0 {
            return Some(0); // degenerate
        }

        // 2. Which columns are pivots, which are free?
        // pivot_row_for_col[j] = Some(row) if x_j is a pivot
        let mut pivot_row_for_col = vec![None; m];
        for (row, p) in pivot_cols.iter().enumerate() {
            if let Some(col) = p {
                if *col < m {
                    pivot_row_for_col[*col] = Some(row);
                }
            }
        }

        let mut free_cols = Vec::new();
        for j in 0..m {
            if pivot_row_for_col[j].is_none() {
                free_cols.push(j);
            }
        }
        let num_params = free_cols.len();

        // 3. Build Expr for each x_j: x_j = constant + Σ coeff_k * param_k
        // param k corresponds to free_cols[k].
        #[derive(Clone)]
        struct Expr {
            coeffs: Vec<Rational>, // len = num_params
            constant: Rational,
        }

        // Initialize all exprs to 0
        let mut exprs: Vec<Expr> = (0..m)
            .map(|_| Expr {
                coeffs: vec![Rational::zero(); num_params],
                constant: Rational::zero(),
            })
            .collect();

        // Free variables: x_{free_cols[k]} = param_k
        for (k, &col) in free_cols.iter().enumerate() {
            exprs[col].coeffs[k] = Rational::one();
            exprs[col].constant = Rational::zero();
        }

        // Helper: expr_p -= factor * expr_other
        impl Expr {
            fn sub_scaled(&mut self, other: &Expr, factor: Rational) {
                for i in 0..self.coeffs.len() {
                    self.coeffs[i] = self.coeffs[i] - factor * other.coeffs[i];
                }
                self.constant = self.constant - factor * other.constant;
            }
        }

        let last_col = m; // augmented [A|b], so b is at column m

        // For each pivot row, solve for that pivot variable in terms of free vars
        for (row, p) in pivot_cols.iter().enumerate() {
            let Some(pivot_col) = p else { continue };
            let pivot_col = *pivot_col;
            if pivot_col >= m {
                continue;
            }

            let rhs = rref_mat[row][last_col];
            let mut expr_p = Expr {
                coeffs: vec![Rational::zero(); num_params],
                constant: rhs,
            };

            // subtract contributions from free variables
            for (k, &free_col) in free_cols.iter().enumerate() {
                let coeff = rref_mat[row][free_col];
                if coeff.is_zero() {
                    continue;
                }
                // x_p = rhs - coeff * x_free
                expr_p.sub_scaled(&exprs[free_col], coeff);
            }

            exprs[pivot_col] = expr_p;
        }

        // 4. We now have x_j(params) = exprs[j].
        //    Next, do a bounded integer search over params to find the min sum of presses.

        // simple bound: params in [0, max_target]
        let max_target = self.target.iter().copied().max().unwrap_or(0).max(0) as i64;

        if num_params == 0 {
            // fully determined, just evaluate once
            let x_vals = (0..m)
                .map(|j| exprs[j].constant.clone())
                .collect::<Vec<_>>();
            let presses = rational_vec_to_presses(&x_vals)?;
            let sum: i64 = presses.iter().sum();
            return Some(sum);
        }

        // generic recursive search over all param combinations
        let mut best_sum: Option<i64> = None;

        fn rational_vec_to_presses(vals: &[Rational]) -> Option<Vec<i64>> {
            let mut out = Vec::with_capacity(vals.len());
            for v in vals {
                // must be >= 0
                if v.num < 0 {
                    return None;
                }
                // must be integer → denominator == 1
                if v.den != 1 {
                    return None;
                }
                out.push(v.num);
            }
            Some(out)
        }

        // recursive search over parameters
        fn search_params(
            param_idx: usize,
            params: &mut [i64],
            exprs: &[Expr],
            max_target: i64,
            best_sum: &mut Option<i64>,
        ) {
            let num_params = params.len();
            if param_idx == num_params {
                // evaluate all x_j
                let m = exprs.len();
                let mut vals = Vec::with_capacity(m);
                for expr in exprs {
                    let mut v = expr.constant;
                    for (k, &p) in params.iter().enumerate() {
                        if !expr.coeffs[k].is_zero() && p != 0 {
                            v = v + expr.coeffs[k] * Rational::from_i64(p);
                        }
                    }
                    vals.push(v);
                }

                let presses = match rational_vec_to_presses(&vals) {
                    Some(p) => p,
                    None => return,
                };

                let sum: i64 = presses.iter().sum();
                if let Some(cur_best) = best_sum {
                    if sum >= *cur_best {
                        return;
                    }
                }
                *best_sum = Some(sum);
                return;
            }

            for v in 0..=max_target {
                params[param_idx] = v;
                // optional: you can add very cheap early pruning here if you like
                search_params(param_idx + 1, params, exprs, max_target, best_sum);
            }
        }

        let mut params = vec![0i64; num_params];
        search_params(0, &mut params, &exprs, max_target, &mut best_sum);

        best_sum
    }
}

fn solve_part2(input: &str) -> i64 {
    let machines: Vec<Machine2> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_machine2)
        .collect();

    let mut total = 0_i64;
    for (idx, m) in machines.iter().enumerate() {
        println!("Machine {idx}:");
        m.print_linear_system();

        let (rref_mat, pivot_cols) = m.rref_augmented();

        println!("RREF [A|b]:");
        for row in &rref_mat {
            println!("{:?}", row);
        }
        println!("pivot_cols = {:?}", pivot_cols);

        let ans = m
            .min_presses_from_rref(&rref_mat, &pivot_cols)
            .expect("no non-negative integer solution for this machine");

        total += ans;
        println!("  min presses = {:?}\n", ans);
    }

    total
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day10.txt");

    let input = read_input("./inputs/day10.txt")?;
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

    const TEST_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 33);
    }
}
