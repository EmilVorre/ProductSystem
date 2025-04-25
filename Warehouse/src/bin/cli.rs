
use product_system::cli::{
    handle_add_product,
    handle_remove_product,
    handle_print_inventory,
    clear_terminal,
};

use product_system::db::create_pool;


#[tokio::main]
async fn main() {
    let pool = create_pool().await;

    loop {
        clear_terminal();
        println!("Please choose an option:");
        println!("1. Add a product");
        println!("2. Remove stock");
        println!("3. Print inventory");
        println!("4. Exit \n");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => handle_add_product(&pool).await,
            2 => handle_remove_product(&pool).await,
            3 => handle_print_inventory(&pool).await,
            4 => break,
            _ => println!("Invalid choice."),
        }
    }
}
