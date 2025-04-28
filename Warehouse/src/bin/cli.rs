use product_system::cli::{
    clear_terminal,
};
use product_system::handlers::{
    handle_add_product_cli,
    handle_remove_product_cli,
    handle_print_inventory_cli,
    handle_print_order_list_cli,
    handle_update_product_cli,
    handle_export_all_products_cli,
    handle_update_product_quantity_cli,
    handle_update_product_minimum_stock_cli,
    handle_update_product_pack_size_cli,
    handle_update_product_distributor_cli,
    handle_backup_database_cli,
    handle_restore_database_cli,
};
use product_system::db::create_pool;
use sqlx::PgPool;
use product_system::cli::utils::wait_for_enter;


#[tokio::main]
async fn main() {
    let pool = create_pool().await;

    loop {
        clear_terminal();
        println!("Please choose an option:");
        println!("1. Add a product");
        println!("2. Remove stock");
        println!("3. Print inventory");
        println!("4. Print order list");
        println!("5. Export all products to CSV");
        println!("6. Update product menu");
        println!("7. Backup menu");
        println!("8. Exit \n");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => handle_add_product_cli(&pool).await,
            2 => handle_remove_product_cli(&pool).await,
            3 => handle_print_inventory_cli(&pool).await,
            4 => handle_print_order_list_cli(&pool).await.expect("REASON"),
            5 => handle_export_all_products_cli(&pool).await.expect("REASON"),
            6 => update_menu(&pool).await,
            7 => backup_menu(&pool).await,
            8 => break,
            _ => println!("Invalid choice."),
        }
    }
}

async fn update_menu(pool: &PgPool) {
    clear_terminal();

    println!("Please choose an option:");
    println!("1. Update product");
    println!("2. Update product quantity");
    println!("3. Update product minimum stock");
    println!("4. Update product pack size");
    println!("5. Update product distributor");
    println!("6. Exit \n");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, try again!");
            wait_for_enter();
            return;
        }        
    };

    let result: Result<(), Box<dyn std::error::Error>> = Ok(());

    match choice {
        1 => handle_update_product_cli(&pool).await.expect("REASON"),
        2 => handle_update_product_quantity_cli(&pool).await.expect("REASON"),
        3 => handle_update_product_minimum_stock_cli(&pool).await.expect("REASON"),
        4 => handle_update_product_pack_size_cli(&pool).await.expect("REASON"),
        5 => handle_update_product_distributor_cli(&pool).await.expect("REASON"),
        7 => {
            println!("Exiting update menu!");
            return;
        },
        _ => {
            println!("Invalid choice, returning to update menu...");
            wait_for_enter();
            return;
        }
    }

    if let Err(e) = result {
        println!("An error occurred: {e}");
        wait_for_enter();
    }
}

async fn backup_menu(pool: &PgPool) {
    clear_terminal();

    println!("Please choose an option:");
    println!("1. Backup database");
    println!("2. Restore database");
    println!("3. Exit \n");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, try again!");
            wait_for_enter();
            return;
        }        
    };

    match choice {
        1 => handle_backup_database_cli(&pool).await.expect("REASON"),
        2 => handle_restore_database_cli(&pool).await.expect("REASON"),
        3 => {
            println!("Exiting backup menu!");
            return;
        },
        _ => {
            println!("Invalid choice, returning to backup menu...");
            wait_for_enter();
            return;
        }
    }
}