use std::io::{self, Write};

fn user_input_await(prompt: &str) -> String {
    println!("\x1B[94m\x1B[1m\n> COPY FOLLOWING TEXT TO CHATBOT\n\x1B[0m\x1B[0m");
    println!("{}", prompt);
    println!("\x1B[91m\x1B[1m\n AFTER PASTING, PRESS: (ENTER), (CTRL+Z), (ENTER) TO FINISH\n\x1B[0m\x1B[0m");
    println!("\x1B[96m\x1B[1m\n> PASTE YOUR RESPONSE:\n\x1B[0m\x1B[0m");

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    input_text.trim().to_string()
}