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

    // let mut t: (bool, i32);
    // for i in 1..=20 {
    //     t = (i % 2 == 0, i);
    //     dbg!(t);
    //     assert!(!t.0);
    // }
    //
    let a = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original: ");
    for row in a {
        println!("{row:?}");
    }

    println!("Transponsed: ");
    for row in transpose(a) {
        println!("{row:?}");
    }
}

fn transpose(mut a: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    for i in 0..3 {
        for j in 0..3 {
            a[i][j] = a[j][i];
        }
    }
    a
}
