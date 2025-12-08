// Working with if else conditions in Rust

fn main() {
    let number = 7;

    if number < 5 {
        println!("The number {} is less than 5", number);
    } else if number == 5 {
        println!("The number {} is equal to 5", number);
    } else {
        println!("The number {} is greater than 5", number);
    }

    // Using if in a let statement
    let condition = true;
    let value = if condition { 10 } else { 20 };
    println!("The value is: {}", value);
}   
