// While loops runs while a condition is true
fn main() {
    let mut number = 30;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    // Stopping while loop with break
    let mut count = 1;

    while count <= 10 {
        println!("Count: {}", count);
        count += 1;
        
    }
}