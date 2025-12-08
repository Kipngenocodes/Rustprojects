fn main() {
    // Scalar Types

    // Integer types
    let x: i32 = 42;
    let y: u32 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    // Floating-point types
    let a: f64 = 3.14159;
    let b: f32 = 2.718;
    println!("64-bit float: {}", a);
    println!("32-bit float: {}", b);

    // Boolean type
    let is_active: bool = true;
    let is_greater: bool = 10 > 5;
    println!("Boolean values: {}, {}", is_active, is_greater);

    // Character type
    let c: char = 'Z';
    let emoji: char = '😀';
    println!("Characters: {}, {}", c, emoji);

    // String type
    let s: &str = "Hello, world!";
    println!("String: {}", s);

    // Compound Types

    // Tuple type
    let tuple: (i32, f64, char) = (500, 6.4, 'A');
    let (x, y, z) = tuple; // destructuring
    println!("Tuple values: {}, {}, {}", x, y, z);
    println!("Access by index: {}", tuple.0);

    // Array type
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array first element: {}", array[0]);
    println!("Array length: {}", array.len());
}
