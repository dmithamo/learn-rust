fn main() {
    // println!("Hello, world!");
    // let mut a = [0; 500];
    //
    // for i in 1..=500 {
    //     a[i - 1] = i as u32;
    // }

    // for i in 1..600 {
    //     print!("{} ", a[i]);
    // }
    // println!("{a:#?}");

    let mut t: (bool, i32);
    for i in 1..=20 {
        t = (i % 2 == 0, i);
        dbg!(t);
    }
}
