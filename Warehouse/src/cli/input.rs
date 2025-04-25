use std::io::{self, Write};

pub fn get_product_details() -> (String, f64, f64) {
    let mut name = String::new();
    let mut price = String::new();
    let mut amount = String::new();

    print!("Enter product name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    print!("Enter product price: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut price).expect("Failed to read line");

    print!("Enter product amount: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut amount).expect("Failed to read line");

    let price: f64 = price.trim().parse().expect("Invalid price");
    let amount: f64 = amount.trim().parse().expect("Invalid amount");

    (name.trim().to_string(), price, amount)
}

pub fn get_stock_details() -> (String, f64) {
    let mut name = String::new();
    let mut amount = String::new();

    print!("Enter product name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    print!("Enter amount: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut amount).expect("Failed to read line");

    let amount: f64 = amount.trim().parse().expect("Invalid amount");

    (name.trim().to_string(), amount)
}