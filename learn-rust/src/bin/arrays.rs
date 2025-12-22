// Arays are collections of values of the same type stored in contiguous memory locations.
fn main() { 
    // creating an array of strings
    let fruits = ["Apple", "Banana", "Orange"];
    println!("First fruit: {}", fruits[0]); // Output: Apple
    println!("Second fruit: {}", fruits[1]); // Output: Banana
    println!("Third fruit: {}", fruits[2]); // Output: Orange

    // Accessinng array elements.
    let numbers = [10, 20, 30, 40, 50];
    println!("First number: {}", numbers[0]); // Output: 10
    println!("Last number: {}", numbers[numbers.len() - 1]); // Output: 50

    // Array length
    let length = fruits.len();  
    println!("Number of fruits: {}", length); // Output: 3

    // Loop through an array
    for fruit in &fruits {
        println!("Fruit: {}", fruit);

    }
    // Priniting the entire array
    println!("All fruits: {:?}", fruits); // Output: ["Apple", "Banana", "Orange"]
}