use std::io::{self, Write};
use sqlx::PgPool;
use crate::db::get_all_products;



pub async fn handle_print_inventory_cli(pool: &PgPool){
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