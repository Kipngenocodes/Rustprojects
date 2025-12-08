// Usefull when you have many choices which is very cumbersome to work with If-Else statements

fn main() {
    let number = 13;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("A number"),
    }
}