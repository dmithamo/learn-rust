use std::collections::HashMap;

fn main() {
    // strings_are_mutable();
    let mut nnames_table = NicknameTable::new();
    nnames_table.insert(
        "Dennis".to_string(),
        vec![
            "D".to_string(),
            "Dennoh".to_string(),
            "TheD".to_string(),
            "T8".to_string(),
            "Dennot8".to_string(),
        ],
    );

    nnames_table.insert(
        "Lorraine".to_string(),
        vec![
            "Lo".to_string(),
            "Lorrainah".to_string(),
            "KhalegiB".to_string(),
        ],
    );

    println!("{nnames_table:?}");
    pass_by_ref_sort(&mut nnames_table);
    println!("{nnames_table:?}");
}

#[allow(dead_code)]
fn strings_are_mutable() {
    let mut names = vec![
        "Dennis".to_string(),
        "Bundi".to_string(),
        "Mithamo".to_string(),
    ];

    println!("{names:?}");
    for n in &mut names {
        n.push('!');
    }
    println!("{names:?}");
}

type NicknameTable = HashMap<String, Vec<String>>;
#[allow(dead_code)]
fn pass_by_ref_sort(n: &mut NicknameTable) {
    for nnames in n.values_mut() {
        nnames.sort();
    }
}
