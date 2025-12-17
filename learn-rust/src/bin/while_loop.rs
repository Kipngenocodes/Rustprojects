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
        println!(" I am counting: {}", count);
        count += 1;

    // Skipping a value with continue
        if count % 2 == 0 {
            continue;
        }
        println!(" Odd number: {}", count);

    // STOPPING the loop when count is greater than 15
        if count > 15 {
            break;
        }

    }
}


