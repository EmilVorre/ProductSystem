use actix_web::{App, HttpServer, web};
use product_system::db::create_pool;
use product_system::handlers::add::add_product_handler;
use product_system::handlers::remove::remove_product_handler;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_product_handler)
            .service(remove_product_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}