#![allow(dead_code)]

use num::complex::Complex;

fn main() {
    // for i in 3..100 {
    //     for j in 6..100 {
    //         println!("gcd({}, {}) = {}", i, j, gcd(i, j));
    //     }
    // }
    // for i in 1..=20 {
    //     print!("collatz_sequence({i}) = ");
    //     println!(" (len {})", collatz_sequence(i));
    // }
    // number_bases();
    // various_powers();
    // assert!(0.1 + 0.2 == 0.3);
    // println!("{}", ((0.1 + 0.2) - 0.3 as f32).abs() <= f32::EPSILON);
    let n = Complex { re: 11, im: 3 };
    println!("{:?}", n);
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

fn number_bases() {
    let two = 2;

    println!(
        "[base 2/binary]={0:b} [base 16/hexadecimal]={0:x} [base 8/octo]={0:o}",
        two
    );
}

fn various_powers() {
    for i in 1..=64 {
        println!("2^{i} = {}", 2_u128.pow(i));
    }
}
