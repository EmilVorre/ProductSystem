use sqlx::PgPool;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use chrono::Local;
use crate::db::products::get_all_products;


pub async fn handle_export_all_products_cli(pool: &PgPool) ->  Result<(), Box<dyn Error>> {
    let products = get_all_products(pool).await?;

    let now = Local::now();
    let date_string = now.format("%Y-%m-%d").to_string();
    let filename = format!("warehouseExport{}.csv", date_string);

    let file = File::create(filename)?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&["id", "name", "price", "quantity", "minimum_stock", "pack_size", "distributor"])?;

    for product in products {
        wtr.write_record(&[
            product.id.to_string(),
            product.name,
            product.price.to_string(),
            product.quantity.to_string(),
            product.minimum_stock.unwrap_or(0.0).to_string(),
            product.pack_size.unwrap_or(1).to_string(),
            product.distributor.unwrap_or_default(),
        ])?;
    }

    wtr.flush()?;

    Ok(())
}