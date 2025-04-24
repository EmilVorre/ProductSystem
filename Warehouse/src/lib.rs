pub mod data_struct;

use std::io::{self, Write};

// Clears the terminal
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("Failed to flush stdout");
}

// Prints the products in the vector
pub fn print_products(products: &Vec<data_struct::products::Product>) {
    for product in products {
        println!("{} - {} - {}", product.name, product.price, product.amount);
    }
    println!("Press Enter to return to the main menu...");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut enter = String::new();
    io::stdin().read_line(&mut enter).expect("Failed to read line");
}

// Adds a product to the vector
pub fn add_product(products: &mut Vec<data_struct::products::Product>, name: &str, price: f64, amount: f64) {
    products.push(data_struct::products::Product::new(name, price, amount));
}

pub fn add_product_with_negativ_stock(products: &mut Vec<data_struct::products::Product>, name: &str, price: f64, amount: f64) {
    products.push(data_struct::products::Product::new(name, price, -amount));
}

// Adds stock to a product
pub fn add_stock(products: &mut Vec<data_struct::products::Product>, name: &str, amount: f64) {
    for product in products.iter_mut() {
        if product.name == name {
            product.amount += amount;
        }
    }
}

// Removes stock from a product
pub fn remove_stock(products: &mut Vec<data_struct::products::Product>, name: &str, amount: f64) {
    for product in products.iter_mut() {
        if product.name == name {
            product.amount -= amount;
        }
    }
}