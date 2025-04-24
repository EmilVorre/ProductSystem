use ProductSystem::data_struct::products::Product;
use ProductSystem::{add_product, add_stock, remove_stock};

// Tests the add_product function
#[test]
fn test_add_product() {
    let mut products = Vec::new();
    add_product(&mut products, "Test_Product", 10.0, 20.0);
    assert_eq!(products.len(), 1);
    assert_eq!(products[0].name, "Test_Product");
    assert_eq!(products[0].price, 10.0);
    assert_eq!(products[0].amount, 20.0);
}

// Tests the add_stock function
#[test]
fn test_add_stock() {
    let mut products = vec![Product::new("Test_Product", 10.0, 20.0)];
    add_stock(&mut products, "Test_Product", 10.0);
    assert_eq!(products[0].amount, 30.0);
}

// Tests the remove_stock function
#[test]
fn test_remove_stock() {
    let mut products = vec![Product::new("Test_Product", 10.0, 20.0)];
    remove_stock(&mut products, "Test_Product", 10.0);
    assert_eq!(products[0].amount, 10.0);
}