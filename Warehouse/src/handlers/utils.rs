use crate::cli::utils::{read_input, wait_for_enter};
use crate::utils::string_utils::clean_string_for_db;
use crate::db::products::get_product_by_name;
use sqlx::PgPool;
use crate::models::products::Product;

pub async fn get_product_with_retry(pool: &PgPool) -> Result<Product, Box<dyn std::error::Error>> {
    loop {
        println!("Enter the name of the product:");
        let raw_name = read_input().await;
        let name = clean_string_for_db(&raw_name);

        match get_product_by_name(pool, &name).await {
            Ok(product) => return Ok(product),
            Err(_) => {
                println!("Product not found. Please try again.");
                wait_for_enter();
            }
        }
    }
}

pub async fn get_product_without_retry(pool: &PgPool) -> Result<Product, Box<dyn std::error::Error>> {
    println!("Enter the name of the product:");
    let raw_name = read_input().await;
    let name = clean_string_for_db(&raw_name);

    match get_product_by_name(pool, &name).await {
        Ok(product) => Ok(product),
        Err(_) => {
            println!("Product not found. Please check the name and try again.");
            wait_for_enter();
            Err("Product not found".into())
        }
    }
}

use std::io::{self, Write};

pub async fn read_number_with_retry<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<T>() {
            Ok(num) => return num,
            Err(e) => {
                println!("Invalid input: {}. Please enter a valid number.", e);
                print!("> ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
