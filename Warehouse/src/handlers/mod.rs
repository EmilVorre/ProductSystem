pub mod add;
pub mod remove;
pub mod print_inventory;

pub use add::{
    add_product_handler,
    handle_add_product_cli,
};
pub use remove::{
    remove_product_handler,
    handle_remove_product_cli,
};
pub use print_inventory::{
    handle_print_inventory_cli,
};