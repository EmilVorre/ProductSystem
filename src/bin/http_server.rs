use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, post};
use product_system::data_struct::products::ProductSale;



#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[post("/product")]
async fn product(product_sale: web::Json<ProductSale>) -> impl Responder {
    let product = product_sale.into_inner(); // Extract the inner Product_sale
    println!(
        "Received product: {} (Quantity: {})", 
        product.product_name, 
        product.quantity
    );
    HttpResponse::Ok().json(product) // Return the product as JSON
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(product)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}