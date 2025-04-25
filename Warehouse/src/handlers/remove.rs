use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::products::ProductSale;
use crate::db::remove_product_stock;

#[post("/remove_product")]
pub async fn remove_product_handler(
    pool: web::Data<PgPool>,
    product_data: web::Json<ProductSale>,
) -> impl Responder {
    let product = product_data.into_inner();

    match remove_product_stock(pool.get_ref(), &product.product_name, product.quantity).await {
        Ok(_) => HttpResponse::Ok().json(product),
        Err(err) => {
            eprintln!("Failed to remove stock: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
