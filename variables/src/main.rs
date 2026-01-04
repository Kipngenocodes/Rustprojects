fn main() {
    let  mut y = 25;
    println!("The value of y is: {}", y);
    // Variables are immutable by default, so we need to use 'mut' to make it mutable
    y = 34;
    println!("The value of y is: {}", y);
}
