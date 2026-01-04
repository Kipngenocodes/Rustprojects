fn main() {
    let  mut y = 25;
    println!("The value of y is: {}", y);
    // Variables are immutable by default, so we need to use 'mut' to make it mutable
    y = 34;
    println!("The value of y is: {}", y);

    // Working on math operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
