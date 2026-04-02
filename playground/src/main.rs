#![allow(dead_code)]

fn main() {
    // for i in 3..100 {
    // for j in 6..100 {
    //     println!("gcd({}, {}) = {}", i, j, gcd(i, j));
    // }
    // }
    // eprintln!("{}", factorial(7));
    for i in 1..=20 {
        print!("collatz_sequence({i}) = ");
        println!(" (len {})", collatz_sequence_len(i));
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b > 0 { gcd(b, a % b) } else { a }
}

fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn collatz_sequence_len(mut n: u32) -> u32 {
    let mut cycles: u32 = 0;

    while n > 1 {
        print!("{n}");
        cycles += 1;
        print!(" ");

        if n.is_multiple_of(2) {
            n /= 2
        } else {
            n = n * 3 + 1
        };
    }

    print!("1");
    cycles + 1
}
