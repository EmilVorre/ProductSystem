#![allow(non_snake_case)]
#![warn(unused_imports)]


mod dataStruct {
    pub mod products;
}

use dataStruct::products::Product;

fn main() {
    let mut products = Vec::new();

    let beer = Product::new("Beer", 2.5, 10.0);
    let cola = Product::new("Cola", 1.5, 5.0);

    products.push(beer);
    products.push(cola);

    for product in products {
        println!("{} - {} - {}", product.name, product.price, product.amount);
    }



}