// Creating a function with parameters in Rust
fn main() {
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    let name = "Alice";
    greet(name);    

    // function with multiple parameters and return value
    // Use the  -> symbol in the function header to show what type of value will be returned.
    // Inside the function, use the return keyword to send the value back:
    fn add(a: i32, b: i32) -> i32 {
        return a + b;

    }
    let sum = add(5, 10);
    println!("The sum is: {}", sum);
}