// Rust Strings and String Manipulation
// String are used to strore text data in Rust. There are two main types of strings in Rust: String and &str (string slice).

// &str type is used to create string literals
// &str is called a syring slicer because it is a reference to a string data stored elsewhere and is used on fixed-size strings.
// String is used to create growable, heap-allocated strings.Vital when a sting can be changed during runtime
fn main() {
    let hello = "Hello, world";
    println!("{}", hello);
    // Creating a String
    let test1 = "Hello World".to_string() ;
    print!("{}", test1);
    let test2 = String::from("Hello Rust");
    println!("{}", test2);

    // Change a  string
    let mut greeting = String::from("Hello");
    greeting.push_str(", World!");
    println!("{}", greeting);  
    // Concatenate strings
    let str1 = String::from("Hello, ");
    let str2 = String::from("World!");      
    let str3 = str1 + &str2;
    println!("{}", str3);
    let str4 = format!("{} {}", "Hello", "Rust");
    println!("{}", str4); 
}