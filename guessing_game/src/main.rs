use std::cmp::Ordering;      // Needed for .cmp()
use std::io;                  // For stdin
use rand::Rng;                // For random number generation

// All imports are correct

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // ← inclusive range 1..=100 ✓

    loop {  // infinite loop until break ✓

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");  // ← good error handling for this simple program

        let guess: u32 = match guess.trim().parse() {  // trim + parse + error handling ✓
            Ok(num) => num,
            Err(_) => continue,           // skips invalid input (very nice!)
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {  // classic usage of cmp + Ordering enum ✓
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;                    // ← exits the loop when correct
            }
        }
    }
}