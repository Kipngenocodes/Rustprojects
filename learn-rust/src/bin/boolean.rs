// Rust Booleans 
fn main (){
    let word_said: bool = true;
    let word_not_said: bool = false;

    println!("Is programming fun? {}", word_said);
    println!("I hate programming because it is not fun? {}", word_not_said);

    // using boolean in conditional statements
    let word_saided: bool = true;
    if word_saided {
        println!("Yes, programming is fun!");
    } else {
        println!("No, programming is not fun.");
    }
} 