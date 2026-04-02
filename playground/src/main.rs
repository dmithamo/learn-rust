#![allow(dead_code)]

fn main() {
    // for i in 3..100 {
    // for j in 6..100 {
    //     println!("gcd({}, {}) = {}", i, j, gcd(i, j));
    // }
    // }
    // eprintln!("{}", factorial(7));
    for i in 1..20 {
        print!("collatz_sequence({i}) = ");
        println!("(len {})", collatz_sequence_len(i));
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

fn collatz_sequence_len(mut n: u32) -> i32 {
    let mut cycles = 0;

    while n > 0 {
        cycles += 1;
        print!("{n} ");

        if n == 1 {
            break;
        }

        print!(" ");

        if n.is_multiple_of(2) {
            n /= 2
        } else {
            n = n * 3 + 1
        };
    }

    cycles
}
