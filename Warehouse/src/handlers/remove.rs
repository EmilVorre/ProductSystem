use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::products::ProductSale;
use crate::db::remove_product_stock;
use crate::cli::input::get_stock_details;
use crate::utils::string_utils::clean_string_for_db;


#[post("/remove_product")]
pub async fn remove_product_handler(
    pool: web::Data<PgPool>,
    product_data: web::Json<ProductSale>,
) -> impl Responder {
    let product = product_data.into_inner();

    match remove_product_stock(pool.get_ref(), &clean_string_for_db(&product.product_name), product.quantity).await {
        Ok(_) => HttpResponse::Ok().json(product),
        Err(err) => {
            eprintln!("Failed to remove stock: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn handle_remove_product_cli(pool: &PgPool){
    let (name, quantity) = get_stock_details();
    if let Err(e) = remove_product_stock(pool, &clean_string_for_db(&name), quantity).await {
        eprintln!("Error removing product: {:?}", e);
    }
}