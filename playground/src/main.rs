fn main() {
    for i in 3..100 {
        for j in 6..100 {
            println!("gcd({}, {}) = {}", i, j, gcd(i, j));
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b > 0 { gcd(b, a % b) } else { a }
}
