// Rust used Ownership system to manage memory.
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped. 
// Baisic Ownership Rules:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main(){
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2 and s1 is no longer valid
    // println!("{}", s1); // This will cause an error because s1 is no longer valid
    println!("{}", s2); // This will work because s2 is the owner of the value

    // Simple data types like integers, characters, and booleans are copied, not moved
    let x = 5;
    let y = x; // x is copied to y
    println!("x = {}, y = {}", x, y); // This will work because x is still valid

    // Clone method to create a deep copy of a value
    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3 is cloned to s4
    println!("s3 = {}, s4 = {}", s3, s4); // This will work because s3 is still valid


}