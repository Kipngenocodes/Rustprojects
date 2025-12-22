// An enum representing different types of messages with different associated data.
enum Message {
    Quit,                       // No associated data
    Move { x: i32, y: i32 },   // Named fields
    Write(String),              // Single associated value
    ChangeColor(i32, i32, i32) // Multiple associated values
}

fn main() {
    // Creating instances of each enum variant
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    // Function to process messages
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => {
                println!("Received Quit message.");
            }
            Message::Move { x, y } => {
                println!("Moving to coordinates: ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Writing message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);
            }
        }
    }

    // Processing each message
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}