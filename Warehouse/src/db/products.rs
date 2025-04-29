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

pub async fn add_product(
    pool: &PgPool,
    name: &str,
    price: f64,
    quantity: f64,
    minimum_stock: Option<f64>,
    pack_size: Option<i32>,
    distributor: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO products (name, price, quantity, minimum_stock, pack_size, distributor)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(name)
    .bind(price)
    .bind(quantity)
    .bind(minimum_stock)
    .bind(pack_size)
    .bind(distributor)
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

pub async fn change_product_quantity(
    pool: &PgPool,
    id: i32,
    new_quantity: Option<f64>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET quantity = COALESCE($2, quantity)
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(new_quantity)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn change_product_minimum_stock(
    pool: &PgPool,
    id: i32,
    new_minimum_stock: Option<f64>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET minimum_stock = COALESCE($2, minimum_stock)
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(new_minimum_stock)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn change_product_pack_size(
    pool: &PgPool,
    id: i32,
    new_pack_size: Option<i32>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET pack_size = COALESCE($2, pack_size)
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(new_pack_size)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn change_product_distributor(
    pool: &PgPool,
    id: i32,
    new_distributor: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET distributor = COALESCE($2, distributor)
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(new_distributor)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn delete_all_products(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM products")
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn insert_product_direct(pool: &PgPool, product: &Product) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO products (name, price, quantity, minimum_stock, pack_size, distributor)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        product.name,
        product.price,
        product.quantity,
        product.minimum_stock,
        product.pack_size,
        product.distributor,
    )
    .execute(pool)
    .await?;
    Ok(())
}