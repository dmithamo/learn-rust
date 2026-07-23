const SUIT_CHARS: [&str; 4] = ["♠", "♥", "♦", "♣"];
const RANK_CHARS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

fn main() {
    let mut deck = new_deck();
    let mut order: Vec<usize> = vec![];
    for i in 0..52 {
        order.push(i);
    }
    print_cards(&order, &deck);

    lcg_fisher_yates(&mut order, 123456_u64);
    print_cards(&order, &deck);

    let player_count = 5;
    let hand_size = 5;
    deal_hand(&mut deck.2, player_count, hand_size);

    println!("Player cards");
    let mut current_player = 0;
    while current_player < player_count {
        print!("Player {} has cards= ", current_player + 1);
        let mut indices = vec![];
        for i in current_player * hand_size..(current_player * hand_size + hand_size) {
            indices.push(order[i as usize]);
        }
        print_cards(&indices, &deck);
        current_player += 1;
    }
}

fn new_deck() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut suits = Vec::with_capacity(52);
    let mut ranks = Vec::with_capacity(52);
    let mut locations = Vec::with_capacity(52);

    for s in 0..4 {
        for r in 0..13 {
            suits.push(s);
            ranks.push(r);
            locations.push(0);
        }
    }

    (suits, ranks, locations)
}

fn card_to_string(suit: u8, rank: u8) -> String {
    format!("{}{}", RANK_CHARS[rank as usize], SUIT_CHARS[suit as usize])
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
fn print_cards(indices: &Vec<usize>, d: &(Vec<u8>, Vec<u8>, Vec<u8>)) {
    let mut count = 0;
    for i in indices {
        count += 1;
        print!("{:>4} ", card_to_string(d.0[*i], d.1[*i]));
        if (count) % 26 == 0 {
            println!();
        }
    }
    println!("");
}

fn deal_hand(locations: &mut Vec<u8>, player_count: u8, hand_size: u8) {
    let total_cards_to_deal = hand_size * player_count;
    let mut total_cards_dealt = 0;
    while total_cards_dealt < total_cards_to_deal {
        locations[total_cards_dealt as usize] = (total_cards_dealt / hand_size) as u8 + 1_u8;
        total_cards_dealt += 1;
    }
}
