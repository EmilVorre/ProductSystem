use sqlx::PgPool;
use crate::models::Product;

pub async fn upsert_product_stock(
    pool: &PgPool,
    name: &str,
    quantity: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO products (name, price, quantity)
        VALUES ($1, 0, $2)
        ON CONFLICT (name) 
        DO UPDATE SET quantity = products.quantity + EXCLUDED.quantity
        "#,
    )
    .bind(name)
    .bind(quantity)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn remove_product_stock(
    pool: &PgPool,
    name: &str,
    quantity: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET quantity = quantity - $2
        WHERE name = $1
        "#,
    )
    .bind(name)
    .bind(quantity)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT name, price, quantity
        FROM products
        ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}