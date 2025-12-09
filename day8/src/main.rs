use std::time::Instant;
use utils::read_input;

#[derive(Debug, Clone, Copy)]
struct Edge {
    dist_sq: i64,
    i: usize,
    j: usize,
}

struct Circuits {
    circuit: Vec<usize>,
}

impl Circuits {
    fn new(n: usize) -> Self {
        // each point starts as its own circuit
        let mut circuit = Vec::with_capacity(n);
        for i in 0..n {
            circuit.push(i);
        }

        Self { circuit }
    }

    // find the root of a node
    fn find(&self, mut x: usize) -> usize {
        while self.circuit[x] != x {
            x = self.circuit[x];
        }
        x
    }

    // merge two circuits
    fn merge(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a != root_b {
            // attach root_b's chain under root_a
            self.circuit[root_b] = root_a;
        }
    }

    // count how big each circuit is
    fn sizes(&self) -> Vec<usize> {
        let n = self.circuit.len();
        let mut counts = vec![0usize; n];

        // walk each node up to its root
        for i in 0..n {
            let root = self.find(i);
            counts[root] += 1;
        }

        // filter out empty ones
        counts.into_iter().filter(|&x| x > 0).collect()
    }
}

fn dist_sq(a: [i64; 3], b: [i64; 3]) -> i64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

fn solve_part1(input: &str, num: usize) -> i64 {
    let points: Vec<[i64; 3]> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();
            [x, y, z]
        })
        .collect();

    let n = points.len();
    let num_edges = n * (n - 1) / 2;
    let mut edges = Vec::with_capacity(num_edges);

    for i in 0..n {
        for j in (i + 1)..n {
            let d2 = dist_sq(points[i], points[j]);
            edges.push(Edge { dist_sq: d2, i, j });
        }
    }

    edges.sort_by_key(|e| e.dist_sq);

    let mut dsu = Circuits::new(points.len());

    for edge in edges.iter().take(num) {
        dsu.merge(edge.i, edge.j);
    }

    let mut sizes = dsu.sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    (sizes[0] * sizes[1] * sizes[2]).try_into().unwrap()
}

fn solve_part2(input: &str) -> i64 {
    let points: Vec<[i64; 3]> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();
            [x, y, z]
        })
        .collect();

    let n = points.len();
    let num_edges = n * (n - 1) / 2;
    let mut edges = Vec::with_capacity(num_edges);

    for i in 0..n {
        for j in (i + 1)..n {
            let d2 = dist_sq(points[i], points[j]);
            edges.push(Edge { dist_sq: d2, i, j });
        }
    }

    edges.sort_by_key(|e| e.dist_sq);

    let mut dsu = Circuits::new(points.len());

    let mut i = 0;

    let mut final_breaker_1 = [0_i64, 0, 0];
    let mut final_breaker_2 = [0_i64, 0, 0];
    while i < edges.len() {
        let edge = &edges[i];
        dsu.merge(edge.i, edge.j);
        i += 1;

        // check if fully connected
        let sizes = dsu.sizes();
        if sizes.iter().any(|&s| s == n) {
            println!(
                "Fully connected after {} edges, final merge was between {} and {}, point {:?}",
                i, edge.i, edge.j, points[edge.j]
            );
            final_breaker_1 = points[edge.i];
            final_breaker_2 = points[edge.j];
            break;
        }
    }

    final_breaker_1[0] * final_breaker_2[0]
}

fn solve(input: &str) -> (i64, i64) {
    let part1 = solve_part1(input, 1000);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Loading input from ./inputs/day8.txt");

    let input = read_input("./inputs/day8.txt")?;
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

    const TEST_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_part1() {
        let result = solve_part1(TEST_INPUT, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 25272);
    }
}
