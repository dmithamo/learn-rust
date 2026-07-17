use std::{hint::black_box, time};

fn main() {
    // println!("std::mem::size_of::<u8>() = {}", std::mem::size_of::<u8>());
    // println!(
    //     "std::mem::size_of::<u16>() = {}",
    //     std::mem::size_of::<u16>()
    // );
    // println!(
    //     "std::mem::size_of::<u32>() = {}",
    //     std::mem::size_of::<u32>()
    // );
    // println!(
    //     "std::mem::size_of::<u64>() = {}",
    //     std::mem::size_of::<u64>()
    // );
    // println!(
    //     "std::mem::size_of::<i32>() = {}",
    //     std::mem::size_of::<i32>()
    // );
    // println!(
    //     "std::mem::size_of::<f32>() = {}",
    //     std::mem::size_of::<f32>()
    // );
    // println!(
    //     "std::mem::size_of::<f64>() = {}",
    //     std::mem::size_of::<f64>()
    // );
    // println!(
    //     "std::mem::size_of::<usize>() = {}",
    //     std::mem::size_of::<usize>()
    // );

    let count = 100_000_000;
    let a: Vec<u8> = vec![1; count];
    let b: Vec<u64> = vec![1; count];

    let sum_u8_time = {
        let start = time::Instant::now();
        black_box(a.iter().sum::<u8>());
        start.elapsed()
    };
    let sum_u64_time = {
        let start = time::Instant::now();
        black_box(b.iter().sum::<u64>());
        start.elapsed()
    };
    println!(
        "Duration for:\n\tsum_u8: {sum_u8_time:?}\n\tsum_u64: {sum_u64_time:?}\n\nRatio: {:?}",
        sum_u64_time.as_nanos() as f32 / sum_u8_time.as_nanos() as f32
    );
}
