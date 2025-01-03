use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Symbol::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Symbol::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Symbol::Name).string().not_null())
                    .col(
                        ColumnDef::new(Symbol::LastPrice)
                            .double()
                            .not_null()
                            .default(100.0),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Order::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Order::SymbolId).string().not_null())
                    .col(ColumnDef::new(Order::Type).string().not_null())
                    .col(ColumnDef::new(Order::Price).float().not_null())
                    .col(ColumnDef::new(Order::Quantity).integer().not_null())
                    .col(ColumnDef::new(Order::Date).date().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("symbol_id_fk")
                            .from(Order::Table, Order::SymbolId)
                            .to(Symbol::Table, Symbol::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("symbol_id_idx")
                    .if_not_exists()
                    .table(Order::Table)
                    .col(Order::SymbolId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Symbol::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Order::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    SymbolId,
    Type,
    Price,
    Quantity,
    Date,
}
#[derive(DeriveIden)]
enum Symbol {
    Table,
    Id,
    Name,
    LastPrice,
}
