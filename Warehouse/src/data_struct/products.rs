use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]  // Added Serde derives for Product
pub struct Product {
    pub name: String,
    pub price: f64,
    pub amount: f64,
}

impl Product {
    pub fn new(name: &str, price: f64, amount: f64) -> Product {
        Product {
            name: name.to_string(),
            price,
            amount,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]  // Made public (pub) if needed elsewhere
pub struct ProductSale {
    pub product_name: String,
    pub quantity: u32,
}