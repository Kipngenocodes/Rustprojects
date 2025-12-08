// working Rust Loop
fn  main (){
    let mut count = 1;
    loop {
        println!("This will repeat forever!");

        if count == 10 {
            break;  
        }
        count += 1;
    }

println!();
    let result = loop {
        println!("I will run until I hit 20");

        if count == 20 {
            break count;
        }
        count += 1;
    };
    println!("The result is {}", result);
}