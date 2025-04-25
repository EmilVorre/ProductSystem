use actix_web::{test, web, App};
use sqlx::{PgPool, postgres::PgPoolOptions};
use serde_json::json;
use product_system::handlers::add::add_product_handler;
use product_system::handlers::remove::remove_product_handler;


// Helper to create test DB pool
async fn create_test_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost/test_db") // Use a test DB!
        .await
        .expect("Failed to connect to test database")
}

#[actix_web::test]
async fn test_add_product_handler_success() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_product_handler)
    ).await;

    let req = test::TestRequest::post()
        .uri("/add_product")
        .set_json(&json!({
            "product_name": "TestCola",
            "quantity": 5.0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_remove_product_handler_success() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(remove_product_handler)
    ).await;

    let req = test::TestRequest::post()
        .uri("/remove_product")
        .set_json(&json!({
            "product_name": "TestCola",
            "quantity": 2.0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[tokio::test]
async fn test_handle_add_and_remove_product() {
    let pool = create_test_pool().await;

    // Simulate CLI input functions
    // Ideally, these would be injected/mocked, so here we assume database-only test logic

    // Add product
    let result = product_system::db::upsert_product_stock(&pool, "TestCola", 10.0).await;
    assert!(result.is_ok());

    // Remove product
    let result = product_system::db::remove_product_stock(&pool, "TestCola", 5.0).await;
    assert!(result.is_ok());

    // Check updated inventory
    let products = product_system::db::get_all_products(&pool).await.unwrap();
    let cola = products.into_iter().find(|p| p.name == "TestCola").unwrap();
    assert_eq!(cola.quantity, 5.0);
}
