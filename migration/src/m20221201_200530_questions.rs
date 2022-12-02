use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Questions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Questions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Questions::Question).string().not_null())
                    .col(ColumnDef::new(Questions::A1).string().not_null())
                    .col(ColumnDef::new(Questions::A2).string().not_null())
                    .col(ColumnDef::new(Questions::Answer).string().not_null())
                    .col(ColumnDef::new(Questions::Category).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Questions::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Questions {
    Table,
    Id,
    Question,
    A1,
    A2,
    Answer,
    Category,
}
