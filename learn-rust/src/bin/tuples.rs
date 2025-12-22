// Tuples are very useful when you want to work with types of  values stored as a single variables in your program.
// Creating a tuples is simple, you just need to enclose the values in parentheses `()` and separate them with commas.



// Create a function which can be used in the main function
fn get_user_info() -> (String, u8) {
    let name = String::from("Alice");
    let age = 30;
    (name, age)
}


fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is:  {}", x);
    println!("The value of z is:  {}", z);


    // You can also access tuple elements directly using dot notation and the index of the element.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // Unpacking tuples
    let another_tup = (300, 3.5, 2); 
    let (a, b, c) = another_tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);    
    println!("The value of c is: {}", c);  

    // accessing tuple returned from function
    let user_info = get_user_info();
    println!("The name is: {}", user_info.0);
    println!("The age is: {}", user_info.1);

}

