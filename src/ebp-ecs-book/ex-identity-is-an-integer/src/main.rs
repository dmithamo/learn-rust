fn main() {
    print_deck(new_deck());
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

fn print_deck(d: (Vec<u8>, Vec<u8>, Vec<u8>)) {
    for i in 0..52 {
        print!("{:>4} ", card_to_string(d.0[i], d.1[i]));
        if (i + 1) % 13 == 0 {
            println!();
        }
    }
    println!("");
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
