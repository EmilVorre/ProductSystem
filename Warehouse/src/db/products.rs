use sqlx::PgPool;
use crate::models::Product;

pub async fn upsert_product_stock(
    pool: &PgPool,
    name: &str,
    quantity: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO products (name, price, quantity, minimum_stock, pack_size, distributor)
        VALUES ($1, 0, $2, 0, 1, '')
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
        SELECT id, name, price, quantity, minimum_stock, pack_size, distributor
        FROM products
        ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn get_products_below_minimum_stock(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT id, name, price, quantity, minimum_stock, pack_size, distributor
        FROM products
        WHERE quantity < COALESCE(minimum_stock, 0)
        ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn change_product_details(
    pool: &PgPool,
    id: i32,
    new_name: Option<&str>,
    new_price: Option<f64>,
    new_minimum_stock: Option<f64>,
    new_pack_size: Option<i32>,
    new_distributor: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET name = COALESCE($2, name),
            price = COALESCE($3, price),
            minimum_stock = COALESCE($4, minimum_stock),
            pack_size = COALESCE($5, pack_size),
            distributor = COALESCE($6, distributor)
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(new_name)
    .bind(new_price)
    .bind(new_minimum_stock)
    .bind(new_pack_size)
    .bind(new_distributor)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_product_by_name(pool: &PgPool, name: &str) -> Result<Product, sqlx::Error> {
    let product = sqlx::query_as!(
        Product,
        r#"
        SELECT id, name, price, quantity, minimum_stock, pack_size, distributor
        FROM products
        WHERE name = $1
        "#,
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}