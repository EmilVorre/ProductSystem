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

    (
        name.trim().to_string(), 
        price, 
        amount
    )
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

    (
        name.trim().to_string(), 
        amount
    )
}

pub fn get_product_details_full() -> (String, f64, f64, f64, i32, String) {
    use std::io::{self, Write};

    let mut name = String::new();
    let mut price = String::new();
    let mut quantity = String::new();
    let mut minimum_stock = String::new();
    let mut pack_size = String::new();
    let mut distributor = String::new();

    print!("Enter product name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Enter product price: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut price).unwrap();

    print!("Enter product quantity: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut quantity).unwrap();

    print!("Enter minimum stock: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut minimum_stock).unwrap();

    print!("Enter pack size: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pack_size).unwrap();

    print!("Enter distributor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut distributor).unwrap();

    (
        name.trim().to_string(),
        price.trim().parse().expect("Invalid price"),
        quantity.trim().parse().expect("Invalid quantity"),
        minimum_stock.trim().parse().expect("Invalid minimum stock"),
        pack_size.trim().parse().expect("Invalid pack size"),
        distributor.trim().to_string(),
    )
}
