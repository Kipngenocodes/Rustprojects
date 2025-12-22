// Vectors are resizable arrays in Rust. They are part of the standard library and can be found in the `std::vec` module.
// Vectors can grow and shrink in size, allowing you to add or remove elements as needed

fn main() {
    // Creating a new vector
    let mut numbers: Vec<i32> = Vec::new();

    // Adding elements to the vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Numbers after adding elements: {:?}", numbers); // Output: [10, 20, 30]

    // Accessing elements in the vector
    println!("First number: {}", numbers[0]); // Output: 10
    println!("Second number: {}", numbers[1]); // Output: 20

    // Removing the last element from the vector
    if let Some(last) = numbers.pop() {
        println!("Removed last number: {}", last); // Output: 30
    }
    println!("Numbers after removing last element: {:?}", numbers); // Output: [10, 20]

    // Getting the length of the vector
    let length = numbers.len();
    println!("Length of the vector: {}", length); // Output: 2

    // Looping through the vector
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Creating a vector with initial values
    let mut fruits = vec!["Apple", "Banana", "Orange"];
    println!("Fruits: {:?}", fruits); // Output: ["Apple", "Banana", "Orange"]

    // Adding a new fruit
    fruits.push("Grapes");
    println!("Fruits after adding Grapes: {:?}", fruits); // Output: ["Apple", "Banana", "Orange", "Grapes"]
}