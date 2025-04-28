use sqlx::PgPool;
use crate::db::get_all_products;
use crate::cli::utils::wait_for_enter;
use crate::utils::string_utils::format_string_for_display_with_capitalization;



pub async fn handle_print_inventory_cli(pool: &PgPool){
    match get_all_products(pool).await {
        Ok(products) => {
            println!("{:<20} | {:<10} | {:<10}", "Product", "Price", "Quantity");
            println!("{:-<44}", "");
            for p in products {
                println!("{:<20} | {:<10.2} | {:<10.2}", format_string_for_display_with_capitalization(&p.name), p.price, p.quantity);
            }
        }
        Err(e) => {
            eprintln!("Error fetching inventory: {:?}", e);
        }
    }

    // Wait for user input before returning to the main menu
    println!("\n");
    wait_for_enter();
    
}