// cOMMON Data Structures in Rust include Arrays, vectors, tuples, and HashMaps.
/*Arrays is a fixed-size collection of elements of the same type. You can't add or remove elements from an array once it's created.
To access an array element, you need to know its index.    */

/*Vecors are resizable arrays that can grow or shrink in size.   */

/* Tuples can hold multiple alues of different types. it is very useful when grouping different types together 
You can access tuple elements by their index. like person.1*/

// HashMaps are key-value pairs that allow you to store and retrieve values based on a unique key. To use HashMaps, you need to import the std::collections::HashMap module.
// Import the HashMap module
use std::collections::HashMap;

fn main(){
    let fruits = ["Apple", "Banana", "Orange"];
    println!("First fruit: {}", fruits[0]);

    // Example of a vector
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    println!("Numbers: {:?}", numbers);

    // Example of a tuple
    let person: (&str, i32, &str, i32) = ("Alice", 30, "Engineer", 1000000);
    println!("Name: {}, Age: {}, Job: {}, Salary: {}", person.0, person.1, person.2, person.3);

    // Example a HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 85);
    scores.insert("Bob", 92);
    println!("Scores: {:?}", scores);
    
}