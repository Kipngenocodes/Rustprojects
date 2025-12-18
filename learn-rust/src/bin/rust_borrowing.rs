// Reference  allows to llook at a value without taking ownership of it.
// References are indicated by & symbol.

fn main() {
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);
}