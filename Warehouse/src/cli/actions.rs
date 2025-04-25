use sqlx::PgPool;
use crate::db::{
    upsert_product_stock,
    remove_product_stock,
    get_all_products,
};
use crate::cli::input::{
    get_stock_details,
    get_product_details,
};
use std::io::{self, Write};


pub async fn handle_add_product(pool: &PgPool){
    let (name, _price, quantity) = crate::cli::input::get_product_details();
    if let Err(e) = upsert_product_stock(pool, &name, quantity).await {
        eprintln!("Error adding/updating product: {:?}", e);
    }
}

pub async fn handle_remove_product(pool: &PgPool){
    let (name, quantity) = get_stock_details();
    if let Err(e) = remove_product_stock(pool, &name, quantity).await {
        eprintln!("Error removing product: {:?}", e);
    }
}

pub async fn handle_print_inventory(pool: &PgPool){
    match get_all_products(pool).await {
        Ok(products) => {
            println!("{:<20} | {:<10} | {:<10}", "Product", "Price", "Quantity");
            println!("{:-<42}", "");
            for p in products {
                println!("{:<20} | {:<10.2} | {:<10.2}", p.name, p.price, p.quantity);
            }
        }
        Err(e) => {
            eprintln!("Error fetching inventory: {:?}", e);
        }
    }

    // Wait for user input before returning to the main menu
    println!("\nPress Enter to return to the main menu...");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut enter = String::new();
    io::stdin().read_line(&mut enter).expect("Failed to read line");
    
}