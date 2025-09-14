use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Recipe::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipe::Title).string().not_null())
                    .col(ColumnDef::new(Recipe::Category).string().null())
                    .col(ColumnDef::new(Recipe::Area).string().null())
                    // JSON column for ingredients
                    .col(ColumnDef::new(Recipe::Ingredients).json().not_null())
                    // instructions, etc.
                    .col(ColumnDef::new(Recipe::Instructions).text().not_null())
                    .col(ColumnDef::new(Recipe::ThumbnailUrl).string().null())
                    .col(ColumnDef::new(Recipe::YoutubeUrl).string().null())
                    .col(
                        ColumnDef::new(Recipe::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Recipe::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipe::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Recipe {
    Table,
    Id,
    Title,
    Category,
    Area,
    Ingredients,
    Instructions,
    ThumbnailUrl,
    YoutubeUrl,
    CreatedAt,
    UpdatedAt,
}
