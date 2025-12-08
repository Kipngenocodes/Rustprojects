// Working with Rust Operators 
// Arithmetic, Comparison, assignment, and Logical Operators

fn main() {
    // Arithmetic Operators
    let a = 10;
    let b = 3;

    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;

    println!("Arithmetic Operations:");
    println!("{} + {} = {}", a, b, sum);
    println!("{} - {} = {}", a, b, difference);
    println!("{} * {} = {}", a, b, product);
    println!("{} / {} = {}", a, b, quotient);
    println!("{} % {} = {}", a, b, remainder);

    // Comparison Operators
    println!("\nComparison Operations:");
    println!("{} == {}: {}", a, b, a == b);
    println!("{} != {}: {}", a, b, a != b);
    println!("{} > {}: {}", a, b, a > b);
    println!("{} < {}: {}", a, b, a < b);
    println!("{} >= {}: {}", a, b, a >= b);
    println!("{} <= {}: {}", a, b, a <= b);

    // Logical Operators
    let x = true;
    let y = false;

    let and_result = x && y;
    let or_result = x || y;
    let not_result = !x;

    println!("\nLogical Operations:");
    println!("{} AND {}: {}", x, y, and_result);
    println!("{} OR {}: {}", x, y, or_result);
    println!("NOT {}: {}", x, not_result);  

    // loguical operators with comparisons
    let p = 5;  
    let q = 10; 
    let logical_and = (p < q) && (a > b);
    let logical_or = (p > q) || (a > b);    
    println!("\nLogical Operations with Comparisons:");
    println!("({} < {}) AND ({} > {}): {}", p, q, a, b, logical_and);
    println!("({} > {}) OR ({} > {}): {}", p, q, a  , b, logical_or);

}