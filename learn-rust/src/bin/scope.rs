// Scope refers to where a variable is allowed to be used.

// A variable only lives inside the block where it was created. A block is anything inside curly braces { }.

fn main() {
    fn my_function() {
        let message = "Hello!";
        println!("{}", message);
    }

    my_function();

//   println!("{}", message); // Error. Try to remove this line
}