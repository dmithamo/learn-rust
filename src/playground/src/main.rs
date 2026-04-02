#![allow(dead_code)]

fn main() {
    // for i in 3..100 {
    //     for j in 6..100 {
    //         println!("gcd({}, {}) = {}", i, j, gcd(i, j));
    //     }
    // }
    for i in 1..=20 {
        print!("collatz_sequence({i}) = ");
        println!(" (len {})", collatz_sequence(i));
    }
}

fn collatz_sequence(mut n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut len = 1;
    while n > 1 {
        len += 1;
        print!("{n} ");
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            n * 3 + 1
        };
    }

    print!("1 ");
    len
}

fn gcd(a: i32, b: i32) -> i32 {
    if b > 0 { gcd(b, a % b) } else { a }
}
