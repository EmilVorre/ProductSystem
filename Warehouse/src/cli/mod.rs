pub mod input;
pub mod actions;
pub mod utils;

pub use actions::{
    handle_add_product,
    handle_remove_product,
    handle_print_inventory,
};

pub use input::{
    get_product_details,
    get_stock_details,
};

pub use utils::{
    clear_terminal,
};