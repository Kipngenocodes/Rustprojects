// Creating a for loop in rust
// When you know exactly how many times you want to loop through a block of code,
//  use the for loop together with the in keyword, instead of a while loop:
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);

    }
}