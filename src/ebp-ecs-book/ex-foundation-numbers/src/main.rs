fn main() {
    println!("std::mem::size_of::<u8>() = {}", std::mem::size_of::<u8>());
    println!(
        "std::mem::size_of::<u16>() = {}",
        std::mem::size_of::<u16>()
    );
    println!(
        "std::mem::size_of::<u32>() = {}",
        std::mem::size_of::<u32>()
    );
    println!(
        "std::mem::size_of::<u64>() = {}",
        std::mem::size_of::<u64>()
    );
    println!(
        "std::mem::size_of::<i32>() = {}",
        std::mem::size_of::<i32>()
    );
    println!(
        "std::mem::size_of::<f32>() = {}",
        std::mem::size_of::<f32>()
    );
    println!(
        "std::mem::size_of::<f64>() = {}",
        std::mem::size_of::<f64>()
    );
    println!(
        "std::mem::size_of::<usize>() = {}",
        std::mem::size_of::<usize>()
    );
}
