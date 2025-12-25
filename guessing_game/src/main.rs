use std::io;
fn main() {
    println!("Guess the number!");

    println!("Pleasue input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //rustc 1.0.0

    println!("You guessed: {}", guess);
}
