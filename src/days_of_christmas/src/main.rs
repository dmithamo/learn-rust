/**
 * Off of ch3 of The Book
 *
 * It is required to write a script that prints out the
 * verses of the song 12 Days of Christmas, using only the knowledge of
 * scalar and compound data types, functions and control flow
 *
 * Here follows my solution
 */

const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "turtle-doves",
    "French hens",
    "calling birds",
    "golden rings",
    "geese a laying",
    "swans a swimming",
    "maids a milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

fn main() {
    let mut i = 1;
    while i <= 12 {
        sing_verse(i);

        i += 1;
    }
}

fn sing_intro(i: usize) {
    println!(
        "On the {}{} day of Christmas\nMy true love gave to me",
        i,
        if i == 3 {
            "rd"
        } else if i == 2 {
            "nd"
        } else if i == 1 {
            "st"
        } else {
            "th"
        },
    );
}

fn sing_gifts(i: usize) {
    let mut stop = i;
    while stop > 0 {
        if stop == 1 && i != 1 {
            println!("And a {}", GIFTS[stop - 1]);
        } else {
            println!("{} {}", stop, GIFTS[stop - 1]);
        }
        stop -= 1;
    }
}

fn sing_verse(verse_number: usize) {
    sing_intro(verse_number);
    sing_gifts(verse_number);
    println!("------------------\n");
}
