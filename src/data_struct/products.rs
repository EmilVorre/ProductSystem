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