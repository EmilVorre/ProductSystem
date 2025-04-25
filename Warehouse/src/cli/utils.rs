use std::io::{self, Write};

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("Failed to flush stdout");
}