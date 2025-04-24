#![warn(unused_imports)]

use product_system::data_struct::products::Product;
use product_system::{add_product, add_stock, remove_stock, print_products};


fn main() {
    let mut products = Vec::new();

    products.push(Product::new("Tuborg_Grøn", 7.0, 100.0));
    products.push(Product::new("Tuborg_Classic", 7.0, 100.0));
    products.push(Product::new("Coca_Cola", 12.0, 50.0));
    products.push(Product::new("Faxe_Kondi", 12.0, 50.0));
    products.push(Product::new("Hvid_Monster", 14.0, 50.0));

    add_product(&mut products, "Hancook_juice", 8.0, 50.0);
    add_stock(&mut products, "Tuborg_Grøn", 50.0);
    remove_stock(&mut products, "Tuborg_Grøn", 10.0);
    print_products(&products);
}