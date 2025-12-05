use std::time::Instant;
use utils::read_input;

#[derive(Debug, Clone)]
pub struct Id {
    pub valid: bool,
    pub id: i64,
}

impl Id {
    pub fn new_part_a(value: i64) -> Self {
        let s = value.to_string();
        let len = s.len();

        // Check the "exactly repeated twice" rule
        let invalid = if len % 2 == 0 {
            let half = len / 2;
            let (first, second) = s.split_at(half);
            first == second
        } else {
            false
        };

        Self {
            valid: !invalid,
            id: value,
        }
    }

    pub fn new_part_b(value: i64) -> Self {
        let s = value.to_string();
        let doubled = format!("{s}{s}");
        let invalid = doubled[1..doubled.len() - 1].contains(&s);
        Self {
            valid: !invalid,
            id: value,
        }
    }
}

fn parse_range(input: &str) -> Result<(i64, i64), String> {
    let (start, end) = input
        .split_once('-')
        .ok_or_else(|| "Range must contain a single '-'".to_string())?;

    let first = start
        .trim()
        .parse::<i64>()
        .map_err(|_| "First part is not a valid number".to_string())?;

    let second = end
        .trim()
        .parse::<i64>()
        .map_err(|_| "Second part is not a valid number".to_string())?;

    Ok((first, second))
}

fn load_input(filename: &str) -> Result<Vec<Id>, std::io::Error> {
    let content = read_input(filename)?;
    let mut ids = Vec::new();

    let ranges = content.split(",");

    for range in ranges {
        match parse_range(range) {
            Ok((start, end)) => {
                let numbers: Vec<i64> = (start..=end).collect();

                for number in numbers {
                    // Uncomment this to use part_a validator
                    // ids.push(Id::new_part_a(number))
                    ids.push(Id::new_part_b(number))
                }
            }
            Err(err) => {
                println!("Invalid range: {err}");
            }
        }
    }

    Ok(ids)
}

fn main() {
    let start = Instant::now();
    println!("Loading ids from ./inputs/day2.txt");

    let mut output: i64 = 0;
    match load_input("./inputs/day2.txt") {
        Ok(ids) => {
            println!("Successfully loaded {} input:", ids.len());
            for id in &ids {
                if !id.valid {
                    // println!("Id: {} is {}", id.id, id.valid);
                    output += id.id;
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading ids: {}", e);
        }
    }
    println!("total value of invalid inputs {}", output);
    let duration = start.elapsed(); // Calculate the elapsed time
    println!("Execution time: {:?}", duration);
}
