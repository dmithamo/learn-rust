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
    let a: Vec<Vec<i32>> = vec![
        vec![101, 102, 103],
        vec![201, 202, 203],
        vec![301, 302, 303],
    ];
    println!("Original: ");
    for row in &a {
        println!("{row:?}");
    }

    println!("Transponsed: ");
    for row in transpose(&a) {
        println!("{row:?}");
    }
}

fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new matrix with swapped dimensions (cols x rows)
    let mut transposed = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}
