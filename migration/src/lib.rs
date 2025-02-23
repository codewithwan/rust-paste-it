pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250223_012638_change_to_ulid;
mod m20250223_013928_add_shortink;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250223_012638_change_to_ulid::Migration),
            Box::new(m20250223_013928_add_shortink::Migration),
        ]
    }
}
