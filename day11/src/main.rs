use std::{collections::VecDeque, time::Instant};
use utils::read_input;

use std::collections::HashMap;

type Device = String;

type DeviceHash = HashMap<Device, Vec<Device>>;

pub fn you_to_out(devices: DeviceHash) -> i64 {
    let mut queue: VecDeque<Device> = VecDeque::new();

    queue.push_back("you".into());

    let mut count = 0;

    while let Some(device) = queue.pop_front() {
        match (device.as_str()) {
            "out" => {
                count += 1;
            }
            _ => {
                if let Some(outputs) = devices.get(device.as_str()) {
                    for output in outputs {
                        queue.push_back(output.clone());
                    }
                }
            }
        }
    }

    count
}

fn has_all_targets(parts: &[String], a: &str, b: &str, c: &str) -> bool {
    parts.iter().any(|x| x == a) && parts.iter().any(|x| x == b) && parts.iter().any(|x| x == c)
}

fn build_device_string(parts: &[String], extra: Option<&str>, start: &str) -> String {
    let mut out: Vec<String> = vec![start.to_string()];

    out.extend(parts[1..].iter().cloned());

    if let Some(extra_val) = extra {
        if !out.iter().any(|d| d == extra_val) {
            out.push(extra_val.to_string());
        }
    }

    out.join(" ")
}

pub fn svr_to_out(devices: DeviceHash) -> i64 {
    fn solve(device: &str, devices: &DeviceHash, memo: &mut HashMap<String, i64>) -> i64 {
        if let Some(&cached) = memo.get(device) {
            return cached;
        }

        let device_with_targets: Vec<Device> = device
            .split_whitespace()
            .map(|tok| tok.to_string())
            .collect();

        let head = device_with_targets[0].as_str();

        if head == "out" {
            let ok = has_all_targets(&device_with_targets, "out", "dac", "fft");
            let result = if ok { 1 } else { 0 };
            memo.insert(device.to_string(), result);
            return result;
        }

        let mut total = 0_i64;

        if let Some(outputs) = devices.get(head) {
            for output in outputs {
                let next = match head {
                    "dac" => build_device_string(&device_with_targets, Some("dac"), output),
                    "fft" => build_device_string(&device_with_targets, Some("fft"), output),
                    _ => build_device_string(&device_with_targets, None, output),
                };

                total += solve(&next, devices, memo);
            }
        }

        memo.insert(device.to_string(), total);
        total
    }

    let mut memo: HashMap<String, i64> = HashMap::new();
    solve("svr", &devices, &mut memo)
}

fn parse_devices(input: &str) -> DeviceHash {
    let mut map: DeviceHash = HashMap::new();

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (left, right) = line
            .split_once(':')
            .expect("line must contain ':' separating input and outputs");

        let src = left.trim().to_string();

        let outputs: Vec<Device> = right
            .split_whitespace()
            .map(|tok| tok.to_string())
            .collect();

        map.insert(src, outputs);
    }

    map
}

fn solve_part1(input: &str) -> i64 {
    let devices = parse_devices(input);
    you_to_out(devices)
}

fn solve_part2(input: &str) -> i64 {
    let devices = parse_devices(input);
    svr_to_out(devices)
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day11.txt");

    let input = read_input("./inputs/day11.txt")?;
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

    const TEST_INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    const TEST_INPUT_2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT_2);
        assert_eq!(result, 2);
    }
}
