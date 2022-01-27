use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 *  The guessing_game program asks for user input,
 * processes that input,
 * and checks that the input is what is expected
 */

fn main() {
    let mut number_of_guesses: u32 = 0;
    let allowable_guesses = 10;
    let correct_value = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();
        number_of_guesses += 1;
        println!(
            "Guess {}",
            if number_of_guesses > 1 {
                "again!"
            } else {
                "the number"
            },
        );
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Numbers only please::::{}", err);
                continue;
            }
        };

        let guesses_left = allowable_guesses - number_of_guesses;
        let guesses_left = format!(
            "{} guess{} left",
            guesses_left,
            if guesses_left == 1 { "" } else { "es" },
        );

        match guess.cmp(&correct_value) {
            Ordering::Equal => {
                println!(
                    "You guessed right! Correct value is {}.\nTook you {}{}!",
                    correct_value,
                    number_of_guesses,
                    if number_of_guesses == 1 {
                        " guess! Genius!"
                    } else if number_of_guesses < 3 {
                        " guesses only!"
                    } else {
                        " guesses! That's too many, if you ask me :)"
                    }
                );
                break;
            }
            Ordering::Greater => {
                println!("Too high! ({})", guesses_left,)
            }
            Ordering::Less => {
                println!("Too low! ({}", guesses_left)
            }
        }

        if number_of_guesses == allowable_guesses {
            println!(
                "You have reached the maximum number of guesses allowed ({}).\nBye!",
                allowable_guesses
            );
            break;
        }
    }
}
