use crate::db::products::{
    get_all_products,
    insert_product_direct,
    delete_all_products,
};
use crate::models::products::Product;
use serde_json;
use sqlx::PgPool;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use chrono::Local;
use crate::cli::utils::{
    wait_for_enter,
    read_input,
};

pub async fn handle_backup_database_cli(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    println!("Backing up database...");
    let products = get_all_products(pool).await?;

    fs::create_dir_all("backups")?;

    let now = Local::now();
    let date_string = now.format("%Y-%m-%d-%H%M%S").to_string();
    let filename = format!("backups/backup-{}.json", date_string);

    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    let data = serde_json::to_string_pretty(&products)?;
    writer.write_all(data.as_bytes())?;
    writer.flush()?;

    println!("Backup completed successfully. File: {}", filename);

    Ok(())
}

pub async fn handle_restore_database_cli(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    println!("Enter the filename to restore from:");

    let filename_input = read_input().await;

    let filename = format!("backups/{}", filename_input.trim());
    let file = File::open(&filename).map_err(|_| {
        println!("File not found: {}", filename);
        std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")
    })?;
    let reader = BufReader::new(file);

    let products: Vec<Product> = serde_json::from_reader(reader)?;

    println!("Found {} products in the backup file.", products.len());
    println!("Do you want to DELETE all existing products before restoring? (y/n)");
    let confirmation = read_input().await;
    if confirmation.to_lowercase() != "y" {
        println!("Restore aborted.");
        return Ok(());
    }

    println!("Backing up existing products...");
    handle_backup_database_cli(pool).await?;

    println!("Deleting all existing products...");
    delete_all_products(pool).await?;

    println!("Restoring products from backup...");
    for product in products {
        insert_product_direct(pool, &product).await?;
    }

    println!("Restore completed successfully.");
    wait_for_enter();

    Ok(())
}