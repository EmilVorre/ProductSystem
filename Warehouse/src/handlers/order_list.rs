use crate::db::products::get_products_below_minimum_stock;
use sqlx::PgPool;
use crate::cli::utils::wait_for_enter;

pub async fn handle_print_order_list_cli(pool: &PgPool) -> Result<(), sqlx::Error> {
    let products = get_products_below_minimum_stock(pool).await?;

    for product in products {
        let min_stock = product.minimum_stock.unwrap_or(0.0);
        let pack_size = product.pack_size.unwrap_or(1) as f64;
        let distributor = product.distributor.clone().unwrap_or_else(|| "Unknown".to_string());

        let shortfall = (min_stock - product.quantity).ceil();
        if shortfall > 0.0 {
            let packs_needed = (shortfall / pack_size).ceil();

            println!(
                "Product: {}, Distributor: {}, Quantity needed: {}, Packs needed: {}",
                product.name, 
                distributor, 
                shortfall,
                packs_needed
            );
        }
    }

    wait_for_enter();

    Ok(())
}
