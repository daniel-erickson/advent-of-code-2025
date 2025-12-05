use std::str::FromStr;

use utils::read_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionType {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Direction {
    pub direction: DirectionType,
    pub steps: u32,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("empty input".to_string());
        }

        let (dir_part, steps_str) = trimmed.split_at(1);
        let direction = match dir_part.chars().next() {
            Some('L') => DirectionType::Left,
            Some('R') => DirectionType::Right,
            Some(c) => return Err(format!("invalid direction: '{}'", c)),
            None => return Err("missing direction".to_string()),
        };

        let steps = steps_str
            .parse::<u32>()
            .map_err(|_| format!("invalid steps: '{}'", steps_str))?;

        Ok(Direction { direction, steps })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rotation {
    /// Current position on the 0..99 dial
    pub current: u32,
    /// Total number of times we've pointed at 0 while moving
    pub zero_hits: u64,
    /// Number of times we've landed exactly on 0
    pub exact_landings: u64,
}

impl Rotation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn starting_at(rotation: u32) -> Self {
        Self {
            current: rotation % 100,
            ..Self::default()
        }
    }

    fn count_zero_hits(&self, steps: u32, distance_to_zero: u32) -> u64 {
        if steps == 0 || distance_to_zero > steps {
            return 0;
        }

        // First hit, then once per full 100 steps after that
        1 + ((steps - distance_to_zero) / 100) as u64
    }

    fn count_zero_hits_right(&self, steps: u32) -> u64 {
        let p = self.current; // 0..99
        // First time we hit 0 going right is after (100 - p) steps (or 100 if p == 0)
        let distance_to_zero = if p == 0 { 100 } else { 100 - p };
        self.count_zero_hits(steps, distance_to_zero)
    }

    fn count_zero_hits_left(&self, steps: u32) -> u64 {
        let p = self.current; // 0..99
        // First time we hit 0 going left is after p steps (or 100 if p == 0)
        let distance_to_zero = if p == 0 { 100 } else { p };
        self.count_zero_hits(steps, distance_to_zero)
    }

    pub fn rotate_right(&mut self, steps: u32) {
        self.zero_hits += self.count_zero_hits_right(steps);
        self.current = (self.current + steps) % 100;
        if self.current == 0 {
            self.exact_landings += 1;
        }
    }

    pub fn rotate_left(&mut self, steps: u32) {
        self.zero_hits += self.count_zero_hits_left(steps);
        let steps_mod = steps % 100;
        self.current = (self.current + 100 - steps_mod) % 100;
        if self.current == 0 {
            self.exact_landings += 1;
        }
    }

    pub fn rotate(&mut self, direction: &Direction) {
        match direction.direction {
            DirectionType::Left => self.rotate_left(direction.steps),
            DirectionType::Right => self.rotate_right(direction.steps),
        }
    }
}

fn load_directions(filename: &str) -> Result<Vec<Direction>, std::io::Error> {
    let lines = read_lines(filename)?;
    let mut directions = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match line.parse::<Direction>() {
            Ok(direction) => directions.push(direction),
            Err(e) => eprintln!("Warning: failed to parse line '{}': {}", line, e),
        }
    }

    Ok(directions)
}

fn main() {
    println!("Loading directions from ./inputs/day1.txt");

    match load_directions("./inputs/day1.txt") {
        Ok(directions) => {
            println!("Successfully loaded {} directions:", directions.len());

            let mut rotation = Rotation::starting_at(50);

            for direction in &directions {
                rotation.rotate(direction);
            }

            println!("Exact landings on zero: {}", rotation.exact_landings);
            println!("Total times pointing at zero: {}", rotation.zero_hits);
        }
        Err(e) => {
            eprintln!("Error loading directions: {}", e);
        }
    }
}
