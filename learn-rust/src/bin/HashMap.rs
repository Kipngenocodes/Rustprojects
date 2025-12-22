// HashMap is a collection Key Value pairs
use std::collections::HashMap;

fn main() {
    // Creating a new HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting key-value pairs into the HashMap
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 30);
    scores.insert(String::from("Charlie"), 40);
    println!("Scores after insertion: {:?}", scores);

    // Accessing values using keys
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score); // Output: 50
    } else {
        println!("Alice's score not found");
    }

    // Updating a value in the HashMap
    scores.insert(String::from("Bob"), 35);
    println!("Scores after updating Bob's score: {:?}", scores);

    // Removing a key-value pair from the HashMap
    scores.remove("Charlie");
    println!("Scores after removing Charlie: {:?}", scores);

    // Iterating over key-value pairs in the HashMap
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Checking the length of the HashMap
    let length = scores.len();
    println!("Number of entries in the HashMap: {}", length); // Output: 2
}