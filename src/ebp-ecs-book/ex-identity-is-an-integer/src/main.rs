fn main() {
    let deck = new_deck();
    let mut indices: Vec<usize> = vec![];
    for i in 0..52 {
        indices.push(i);
    }
    print_deck(indices.clone(), deck.clone());

    lcg_fisher_yates(&mut indices, 123456_u64);
    print_deck(indices, deck);
}

fn new_deck() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut suits: Vec<u8> = vec![];
    let mut ranks: Vec<u8> = vec![];
    let mut locations: Vec<u8> = vec![];

    for i in 0..52 {
        let card_suit: u8 = i % 4;
        suits.push(card_suit);

        let card_rank: u8 = i % 13;
        ranks.push(card_rank);

        locations.push(0);
    }

    (suits, ranks, locations)
}

fn card_to_string(suit: u8, rank: u8) -> String {
    let rank_str = match rank {
        0 => format!("A"),
        10 => format!("J"),
        11 => format!("K"),
        12 => format!("Q"),
        num if num < 10 => format!("{}", num + 1),
        _ => todo!(),
    };
    match suit {
        0 => format!("{rank_str}♠"),
        1 => format!("{rank_str}❤"),
        2 => format!("{rank_str}♣"),
        3 => format!("{rank_str}♦"),
        _ => todo!(),
    }
}

// TODO: Endeavour to understand this Fisher_Yates and LCG stuff
fn lcg_fisher_yates(v: &mut Vec<usize>, mut seed: u64) {
    // Tiny one-liner LCG returning a pseudo-random usize
    let mut lcg = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 32) as usize
    };

    // Fisher-Yates shuffle algorithm
    for i in (1..v.len()).rev() {
        v.swap(i, lcg() % (i + 1));
    }
}

fn print_deck(indices: Vec<usize>, d: (Vec<u8>, Vec<u8>, Vec<u8>)) {
    println!("suits={:?}\nranks={:?}\nlocations={:?}\n\n", d.0, d.1, d.2);

    let mut count = 0;
    for i in indices {
        count += 1;
        print!("{:>4} ", card_to_string(d.0[i], d.1[i]));
        if (count) % 13 == 0 {
            println!();
        }
    }
    println!("");
}
