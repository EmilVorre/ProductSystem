use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenvy::dotenv;

pub async fn create_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool")
}
