use std::time::{self, Duration};

fn main() {
    let counts: Vec<usize> = vec![
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ];
    let mut durations: Vec<(usize, Duration, Duration)> = vec![];
    for c in counts {
        durations.push((c, sequential_access_time(c), random_access_timing(c)));
    }

    let print_width = 15;
    println!("Duration table\n");
    println!(
        "{:<print_width$}{:>print_width$}{:>print_width$}",
        "Count", "Sequential", "Random",
    );
    println!("{}", "_".repeat(print_width * 3));
    for d in durations {
        println!(
            "{:<print_width$?}{:>print_width$?}{:>print_width$?}",
            d.0, d.1, d.2
        );
    }
}

fn sequential_access_time(count: usize) -> Duration {
    let a = vec![1 as i64; count];

    let start = time::Instant::now();
    let _ = a.iter().sum::<i64>();

    (time::Instant::now() - start) / count as u32
}

fn random_access_timing(count: usize) -> Duration {
    let a = vec![1 as u64; count];
    let mut _sum = 0u64;

    let mut indices: Vec<usize> = vec![];
    for i in 0..count {
        indices.push(i as usize);
    }

    let start = time::Instant::now();
    for &j in &indices {
        _sum += a[j];
    }

    (time::Instant::now() - start) / count as u32
}
