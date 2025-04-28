use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    pub minimum_stock: Option<f64>,
    pub pack_size: Option<i32>,
    pub distributor: Option<String>,
}

impl Product {
    pub fn new(name: &str, price: f64, quantity: f64) -> Product {
        Product {
            id: 0, // dummy value, will be set by the database
            name: name.to_string(),
            price,
            quantity,
            minimum_stock: Some(0.0),
            pack_size: Some(1),
            distributor: Some(String::new()), 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSale {
    pub product_name: String,
    pub quantity: f64,
}