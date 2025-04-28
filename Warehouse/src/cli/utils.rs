use std::io::{self, Write};

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("Failed to flush stdout");
}

pub async fn read_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Make sure prompt shows immediately
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}