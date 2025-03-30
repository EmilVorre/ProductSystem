use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, post};
use product_system::data_struct::{products::ProductSale, products::Product};
use product_system::{add_stock, add_product, remove_stock, add_product_with_negativ_stock};
use std::sync::Mutex;

// Use Mutex for thread-safe mutable access
static INVENTORY: Mutex<Vec<Product>> = Mutex::new(Vec::new());

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[post("/add_product")]
async fn add_product_handler(product_data: web::Json<ProductSale>) -> impl Responder {
    let product = product_data.into_inner();
    println!(
        "Received product: {} (Quantity: {})", 
        product.product_name, 
        product.quantity
    );

    let mut inventory = INVENTORY.lock().unwrap();
    
    if inventory.iter().any(|p| p.name == product.product_name) {
        add_stock(&mut inventory, &product.product_name, product.quantity as f64);
        println!("Stock updated for {}, new stock is: {}", 
            product.product_name, 
            inventory.iter()
                .find(|p| p.name == product.product_name)
                .unwrap()
                .amount
        );
    } else {
        add_product(&mut inventory, &product.product_name, 0.0, product.quantity as f64);
        println!("New product added: {}, stock set to: {}", 
            product.product_name,
            inventory.iter()
                .find(|p| p.name == product.product_name)
                .unwrap()
                .amount
        );
    }

    HttpResponse::Ok().json(product)
}

#[post("/remove_product")]
async fn remove_product_handler(product_data: web::Json<ProductSale>) -> impl Responder {
    let product = product_data.into_inner();
    println!(
        "Received product: {} (Quantity: {})", 
        product.product_name, 
        product.quantity
    );

    let mut inventory = INVENTORY.lock().unwrap();
    
    if inventory.iter().any(|p| p.name == product.product_name) {
        remove_stock(&mut inventory, &product.product_name, product.quantity as f64);
        println!("Stock updated for {}, new stock is: {}", 
            product.product_name, 
            inventory.iter()
                .find(|p| p.name == product.product_name)
                .unwrap()
                .amount
        );
    } else {
        add_product_with_negativ_stock(&mut inventory, &product.product_name, 0.0, product.quantity as f64);
        println!("Product {} not found in inventory.", product.product_name);
    }

    HttpResponse::Ok().json(product)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(add_product_handler)
            .service(remove_product_handler)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}