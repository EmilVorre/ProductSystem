use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::products::{ProductSale};
use crate::db::upsert_product_stock;


#[post("/add_product")]
pub async fn add_product_handler(
    pool: web::Data<PgPool>,
    product_data: web::Json<ProductSale>,
) -> impl Responder {
    let product = product_data.into_inner();

    match upsert_product_stock(pool.get_ref(), &product.product_name, product.quantity).await {
        Ok(_) => HttpResponse::Ok().json(product),
        Err(e) => {
            eprintln!("Error adding/updating product: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}