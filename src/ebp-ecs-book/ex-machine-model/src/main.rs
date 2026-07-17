use rand::{self, seq::SliceRandom};
use std::time::{self};
use thousands::Separable;

fn main() {
    let counts: Vec<usize> = vec![
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        // 1_000_000_000,
    ];
    let mut durations: Vec<(usize, f64, f64)> = vec![];
    for c in counts {
        durations.push((c, sequential_access_time(c), random_access_timing(c)));
    }

    let print_width = 24;
    println!("Machine model exercise: Duration for array access");
    println!(
        "{:<print_width$}{:>print_width$}{:>print_width$}",
        "Count", "Sequential(ns)", "Random(ns)",
    );
    println!("{}", "_".repeat(print_width * 3));
    for d in durations {
        println!(
            "{:<print_width$}{:>print_width$.2?}{:>print_width$.2?}",
            (d.0).separate_with_commas(),
            d.1,
            d.2
        );
    }
}

fn sequential_access_time(count: usize) -> f64 {
    let a = vec![1 as u64; count];

    let start = time::Instant::now();
    std::hint::black_box(a.iter().sum::<u64>());

    (time::Instant::now() - start).as_nanos() as f64 / count as f64
}

fn random_access_timing(count: usize) -> f64 {
    let a = vec![1 as u64; count];
    let mut _sum = 0u64;

    let mut indices: Vec<usize> = (0..count).collect();
    let mut rng = rand::rng();
    indices.shuffle(&mut rng);

    let start = time::Instant::now();
    for &j in &indices {
        _sum += a[j];
    }

    std::hint::black_box(_sum);
    (time::Instant::now() - start).as_nanos() as f64 / count as f64
}
