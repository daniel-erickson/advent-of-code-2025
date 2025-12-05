use std::time::Instant;
use utils::read_lines;

#[derive(Debug, Clone)]
pub struct BatteryBank {
    pub batteries: Vec<u8>,
    pub largest_charge: u64,
}

fn pick_n_digits(digits: &[u8], n: usize) -> (Vec<u8>, Vec<usize>) {
    let mut chosen = Vec::with_capacity(n);
    let mut indices = Vec::with_capacity(n);

    let mut start = 0;

    for pos in 0..n {
        let remaining_slots = n - pos;
        let last_allowed = digits.len() - remaining_slots;

        let mut best_idx = start;
        let mut best_val = digits[start];

        for (offset, &v) in digits[start..=last_allowed].iter().enumerate() {
            let i = start + offset;
            if v > best_val {
                best_val = v;
                best_idx = i;
            }
        }

        chosen.push(best_val);
        indices.push(best_idx);

        start = best_idx + 1;
    }

    (chosen, indices)
}

impl BatteryBank {
    pub fn build(input: &str, n: usize) -> Result<Self, String> {
        let batteries: Vec<u8> = input
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| format!("Non digit: {c}"))
                    .map(|d| d as u8)
            })
            .collect::<Result<_, _>>()?;

        let (chosen_digits, _indices) = pick_n_digits(&batteries, n);

        let largest_charge = chosen_digits
            .iter()
            .fold(0_u64, |acc, &d| acc * 10 + d as u64);

        Ok(Self {
            batteries,
            largest_charge,
        })
    }
}

fn lines_to_battery_banks(lines: &[String], n: usize) -> Result<Vec<BatteryBank>, String> {
    let mut banks = Vec::new();

    for line in lines {
        match BatteryBank::build(line, n) {
            Ok(bank) => banks.push(bank),
            Err(e) => eprintln!("Warning: failed to parse line '{}': {}", line, e),
        }
    }

    Ok(banks)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading batteries from input.txt");

    let mut two_digit_bank_output: u64 = 0;
    let mut twelve_digit_bank_output: u64 = 0;

    let lines = read_lines("./inputs/day3.txt")?;

    // Part A
    match lines_to_battery_banks(&lines, 2) {
        Ok(banks) => {
            println!("Successfully loaded {} banks:", banks.len());

            for bank in &banks {
                two_digit_bank_output += bank.largest_charge;
            }
        }
        Err(e) => {
            eprintln!("Error loading banks: {}", e);
        }
    }

    // Part B
    match lines_to_battery_banks(&lines, 12) {
        Ok(banks) => {
            println!("Successfully loaded {} banks:", banks.len());

            for bank in &banks {
                twelve_digit_bank_output += bank.largest_charge;
            }
        }
        Err(e) => {
            eprintln!("Error loading banks: {}", e);
        }
    }

    println!(
        "total value of 2 digit bank charges {}",
        two_digit_bank_output
    );

    println!(
        "total value of 12 digit bank charges {}",
        twelve_digit_bank_output
    );

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    Ok(())
}
