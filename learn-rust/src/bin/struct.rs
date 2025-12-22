// A struct is a custom data type that lets you package related values together.
// Each value in a struct is called a field.
// Structs are useful for creating complex data types that can be used to group together related data.

fn main() {
    // Defining a struct
    struct Person {
        name: String,
        age: u8,
    }

    // Creating an instance of the struct
    let mut person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Accessing struct fields
    println!("Name: {}", person1.name); // Output: Alice
    println!("Age: {}", person1.age);   // Output: 30

    // Modifying struct fields
    person1.age += 1;
    println!("Updated Age: {}", person1.age); // Output: 31

    // Creating another instance of the struct
    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    println!("Name: {}, Age: {}", person2.name, person2.age); // Output: Name: Bob, Age: 25
}