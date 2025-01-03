use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PositionWallet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PositionWallet::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PositionWallet::CodigoAtivo)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PositionWallet::Quantidade)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PositionWallet::PrecoMedio)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PositionWallet::ValorTotal)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PositionWallet::DataUltimaAtualizacao)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PositionWallet::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum PositionWallet {
    Table,
    Id,
    CodigoAtivo,
    Quantidade,
    PrecoMedio,
    ValorTotal,
    DataUltimaAtualizacao,
}
