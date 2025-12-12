use std::process::Command;
use std::time::Instant;

fn parse_internal_time(output: &str) -> Option<f64> {
    // Look for "Execution time: <value><unit>"
    for line in output.lines() {
        if line.contains("Execution time:") {
            // Extract the time portion
            if let Some(time_part) = line.split("Execution time:").nth(1) {
                let time_str = time_part.trim();

                // Handle different formats: "123.456ms", "1.234s", "789µs", "1.234567s"
                if time_str.ends_with("ms") {
                    let value = time_str.trim_end_matches("ms").parse::<f64>().ok()?;
                    return Some(value);
                } else if time_str.ends_with("µs") {
                    let value = time_str.trim_end_matches("µs").parse::<f64>().ok()?;
                    return Some(value / 1000.0); // Convert µs to ms
                } else if time_str.ends_with('s') {
                    let value = time_str.trim_end_matches('s').parse::<f64>().ok()?;
                    return Some(value * 1000.0); // Convert s to ms
                }
            }
        }
    }
    None
}

fn run_day(day: u8) -> Result<(u128, Option<f64>, String), String> {
    let binary_path = format!("./target/release/day{}", day);

    let start = Instant::now();
    let output = Command::new(&binary_path)
        .output()
        .map_err(|e| format!("Failed to execute day{}: {}", day, e))?;
    let duration = start.elapsed();

    if !output.status.success() {
        return Err(format!("day{} exited with non-zero status", day));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let internal_time = parse_internal_time(&stdout);

    Ok((duration.as_millis(), internal_time, stdout))
}

fn main() {
    println!("Advent of Code 2025 - Performance Table");
    println!("========================================\n");

    // Days to run (excluding day 10)
    let days = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11];

    let mut results = Vec::new();

    for day in days {
        match run_day(day) {
            Ok((external_ms, internal_ms, _output)) => {
                results.push((day, external_ms, internal_ms));
            }
            Err(e) => {
                println!("Day {:2}: ERROR - {}", day, e);
            }
        }
    }

    // Print table header
    println!("┌──────┬──────────────┐");
    println!("│ Day  │ Time         │");
    println!("├──────┼──────────────┤");

    let mut total_internal = 0.0;

    for (day, _external_ms, internal_ms_opt) in &results {
        if let Some(internal_ms) = internal_ms_opt {
            total_internal += internal_ms;
            println!("│ {:>4} │ {:>9.2} ms │", day, internal_ms);
        } else {
            println!("│ {:>4} │ {:>12} │", day, "N/A");
        }

        // Add day 10 placeholder after day 9
        if *day == 9 {
            println!("│   10 │  (unfinished)│");
        }
    }

    println!("├──────┼──────────────┤");
    println!("│Total │ {:>9.2} ms │", total_internal);
    println!("└──────┴──────────────┘");
}
