use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::products::{ProductSale};
use crate::db::{
    upsert_product_stock,
    add_product,
};
use crate::utils::string_utils::clean_string_for_db;
use crate::cli::input::{
    get_product_details_full,
};


#[post("/add_product")]
pub async fn add_product_handler(
    pool: web::Data<PgPool>,
    product_data: web::Json<ProductSale>,
) -> impl Responder {
    let product = product_data.into_inner();

    match upsert_product_stock(pool.get_ref(), &clean_string_for_db(&product.product_name), product.quantity).await {
        Ok(_) => HttpResponse::Ok().json(product),
        Err(e) => {
            eprintln!("Error adding/updating product: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn handle_add_product_cli(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let (name, price, quantity, minimum_stock, pack_size, distributor) = get_product_details_full();
    let clean_distributor = clean_string_for_db(&distributor);
    if let Err(e) = add_product(
        pool, 
        &clean_string_for_db(&name), 
        price, 
        quantity, 
        Some(minimum_stock), 
        Some(pack_size), 
        Some(&clean_distributor.as_str())
    ).await {
        eprintln!("Error adding product: {:?}", e);
    }

    Ok(())
}