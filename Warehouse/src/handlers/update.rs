use sqlx::PgPool;
use crate::db::products::{
    change_product_details,
    change_product_distributor,
    change_product_minimum_stock,
    change_product_pack_size,
    change_product_quantity,
};
use crate::cli::utils::{
    read_input,
    wait_for_enter,
};
use crate::utils::string_utils::clean_string_for_db;
use crate::handlers::utils::{
    get_product_with_retry,
    read_number_with_retry,
};



pub async fn handle_update_product_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let product = get_product_with_retry(pool).await?;

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
    wait_for_enter();

    Ok(())
}


pub async fn handle_update_product_quantity_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let product = get_product_with_retry(pool).await?;

    println!("Current product details: {:?}", product);

    println!("Enter new quantity (current: {}):", product.quantity);
    let new_quantity: f64 = read_number_with_retry().await;

    println!("Updating product quantity...");
    change_product_quantity(
        pool,
        product.id,
        Some(new_quantity),
    )
    .await?;

    println!("Product quantity updated successfully.");
    wait_for_enter();

    Ok(())
}

pub async fn handle_update_product_minimum_stock_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let product = get_product_with_retry(pool).await?;

    println!("Current product details: {:?}", product);

    println!("Enter new minimum stock (current: {}):", product.minimum_stock.unwrap_or(0.0));
    let new_minimum_stock: f64 = read_number_with_retry().await;

    println!("Updating product minimum stock...");
    change_product_minimum_stock(
        pool,
        product.id,
        Some(new_minimum_stock),
    )
    .await?;

    println!("Product minimum stock updated successfully.");
    wait_for_enter();

    Ok(())
}

pub async fn handle_update_product_pack_size_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let product = get_product_with_retry(pool).await?;

    println!("Current product details: {:?}", product);

    println!("Enter new pack size (current: {}):", product.pack_size.unwrap_or(1));
    let new_pack_size: i32 = read_number_with_retry().await;


    println!("Updating product pack size...");
    change_product_pack_size(
        pool,
        product.id,
        Some(new_pack_size),
    )
    .await?;

    println!("Product pack size updated successfully.");
    wait_for_enter();

    Ok(())
}

pub async fn handle_update_product_distributor_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let product = get_product_with_retry(pool).await?;

    println!("Current product details: {:?}", product);

    println!("Enter new distributor (current: {}):", product.distributor.as_deref().unwrap_or("None"));
    let raw_new_distributor = read_input().await;
    let new_distributor = clean_string_for_db(&raw_new_distributor);

    println!("Updating product distributor...");
    change_product_distributor(
        pool,
        product.id,
        Some(&new_distributor),
    )
    .await?;

    println!("Product distributor updated successfully.");
    wait_for_enter();

    Ok(())
}