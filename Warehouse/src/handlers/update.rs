use sqlx::PgPool;
use crate::db::products::{get_product_by_name, change_product_details};
use crate::cli::utils::read_input;
use crate::utils::string_utils::clean_string_for_db;
use std::io;

pub async fn handle_update_product_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the name of the product to update:");
    let raw_name = read_input().await;
    let name = clean_string_for_db(&raw_name);

    let product = get_product_by_name(pool, &name).await?;

    println!("Current product details: {:?}", product);

    println!("Enter new details (leave blank to keep current value):");

    println!("New name (current: {}):", product.name);
    let raw_new_name = read_input().await;
    let new_name = clean_string_for_db(&raw_new_name);

    println!("New price (current: {}):", product.price);
    let new_price = read_input().await.parse::<f64>().ok();

    println!("New minimum stock (current: {}):", product.minimum_stock.unwrap_or(0.0));
    let new_minimum_stock = read_input().await.parse::<f64>().ok();

    println!("New pack size (current: {}):", product.pack_size.unwrap_or(1));
    let new_pack_size = read_input().await.parse::<i32>().ok();

    println!("New distributor (current: {}):", product.distributor.as_deref().unwrap_or("None"));
    let raw_new_distributor = read_input().await;
    let new_distributor = clean_string_for_db(&raw_new_distributor);

    println!("Updating product...");
    change_product_details(
        pool,
        product.id,
        Some(&new_name),
        new_price,
        new_minimum_stock,
        new_pack_size,
        Some(&new_distributor),
    )
    .await?;

    println!("Product updated successfully.");
    println!("Press Enter to return to the main menu...");
    let mut enter = String::new();
    io::stdin().read_line(&mut enter).expect("Failed to read line");

    Ok(())
}
