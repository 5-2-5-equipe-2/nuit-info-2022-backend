#[cfg(debug_assertions)]
use dotenvy::dotenv;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let fallback = "sqlite:./db?mode=rwc";

    match std::env::var("DATABASE_URL") {
        Ok(val) => {
            println!("Using DATABASE_URL: {}", val);
        }
        Err(_) => {
            std::env::set_var("DATABASE_URL", fallback);
            println!("Set DATABASE_URL: {}", fallback);
        }
    };

    cli::run_cli(migration::Migrator).await;
}
