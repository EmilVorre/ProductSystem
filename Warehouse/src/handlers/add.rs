use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::products::{ProductSale};
use crate::db::upsert_product_stock;
use crate::utils::string_utils::clean_string_for_db;
use crate::cli::input::{
    get_product_details,
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

pub async fn handle_add_product_cli(pool: &PgPool){
    let (name, _price, quantity) = get_product_details();
    if let Err(e) = upsert_product_stock(pool, &clean_string_for_db(&name), quantity).await {
        eprintln!("Error adding/updating product: {:?}", e);
    }
}