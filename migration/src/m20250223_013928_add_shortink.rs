use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Paste::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Paste::Id)
                            .string_len(26) // ULID length
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        text(Paste::ShortLink)
                            .string_len(26) // Increase length to 26
                            .not_null()
                            .unique_key(),
                    )
                    .col(text(Paste::Content))
                    .col(
                        timestamp_with_time_zone(Paste::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Paste::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Paste {
    Table,
    Id,
    ShortLink,
    Content,
    CreatedAt,
}
