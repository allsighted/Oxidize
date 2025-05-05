/* Number Guessing Game
1. Input range of numbers to guess from.
2. Input your guess. 
3. Action:
```
    Input the guessing range: a
    Not a valid u8, try again
    Input the guessing range: 5
    GUESS: a
    Not a valid u8, try again
    GUESS: 6
    Don't enter more than the set range, try again...
    GUESS: 5
    NOT 5, you have 2 tries left...
    GUESS: 4
    NOT 4, you have 1 tries left...
    GUESS: 3
    NOT 3, you have 0 tries left...
    It was 1
```
*/

use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    // print!("Input the guessing range(1-255: ");
    // io::stdout().flush().unwrap();
    let range: u8 = get_u8_from_user("Input the guessing range: ");

    let to_guess: u8 = rand::rng().random_range(1..=range);

    let mut try_count = 3;
    while try_count > 0 {
        // print!("GUESS: ");
        // io::stdout().flush().unwrap();
        let guess: u8 = loop {
            let input = get_u8_from_user("GUESS: ");
            if input <= range {
                break input;
            }
            println!("Don't enter more than the set range, try again...");
        };

        if guess == to_guess {
            println!("Congrats, U GUESSED {} correct!", to_guess);
            return
        } else {
            println!("NOT {}, you have {} tries left...", guess, try_count-1);
        }

        try_count -= 1;
    }

    println!("It was {}", to_guess);

}

fn get_u8_from_user(prompt: &str) -> u8 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Failed to read input: {}", e);
            continue;
        }

        match input.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Not a valid u8, try again");
                continue;
            }
        }
    }
}
