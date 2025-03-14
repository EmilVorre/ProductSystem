pub mod data_struct {
    pub mod products {
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
    }
}

pub fn print_products(products: &Vec<data_struct::products::Product>) {
    for product in products {
        println!("{} - {} - {}", product.name, product.price, product.amount);
    }
}

pub fn add_product(products: &mut Vec<data_struct::products::Product>, name: &str, price: f64, amount: f64) {
    products.push(data_struct::products::Product::new(name, price, amount));
}

pub fn add_stock(products: &mut Vec<data_struct::products::Product>, name: &str, amount: f64) {
    for product in products.iter_mut() {
        if product.name == name {
            product.amount += amount;
        }
    }
}

pub fn remove_stock(products: &mut Vec<data_struct::products::Product>, name: &str, amount: f64) {
    for product in products.iter_mut() {
        if product.name == name {
            product.amount -= amount;
        }
    }
}