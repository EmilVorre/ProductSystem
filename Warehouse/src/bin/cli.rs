#![warn(unused_imports)]

use product_system::{add_product, add_stock, remove_stock, print_products, clear_terminal};
use std::io::{self, Write};

fn main() {
    let mut products = Vec::new();

    loop {
        clear_terminal();

        println!("Please choose an option:");
        println!("1. Add a product");
        println!("2. Add stock");
        println!("3. Remove stock");
        println!("4. Print inventory");
        println!("5. Exit \n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let (name, price, amount) = get_product_details();
                add_product(&mut products, &name, price, amount);
            }
            2 => {
                let (name, amount) = get_stock_details();
                add_stock(&mut products, &name, amount);
            }
            3 => {
                let (name, amount) = get_stock_details();
                remove_stock(&mut products, &name, amount);
            }
            4 => {
                clear_terminal();
                print_products(&products);
            }
            5 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_product_details() -> (String, f64, f64) {
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

fn get_stock_details() -> (String, f64) {
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