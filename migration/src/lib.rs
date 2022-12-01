pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20221127_134800_create_user_table;
mod m20221127_135535_create_scope_table;
mod m20221201_200530_questions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20221127_134800_create_user_table::Migration),
            Box::new(m20221127_135535_create_scope_table::Migration),
            Box::new(m20221201_200530_questions::Migration),
        ]
    }
}
