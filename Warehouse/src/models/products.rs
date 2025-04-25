use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub quantity: f64,
}

impl Product {
    pub fn new(name: &str, price: f64, quantity: f64) -> Product {
        Product {
            name: name.to_string(),
            price,
            quantity,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSale {
    pub product_name: String,
    pub quantity: f64,
}