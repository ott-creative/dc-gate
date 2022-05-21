use entity::eth_tx::*;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table_eth_txs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Uuid).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Column::From).string().not_null())
                    .col(ColumnDef::new(Column::To).string().not_null())
                    .col(ColumnDef::new(Column::Value).big_unsigned().not_null())
                    .col(ColumnDef::new(Column::GasPrice).big_unsigned())
                    .col(ColumnDef::new(Column::TxHash).string())
                    .col(
                        ColumnDef::new(Column::ConfirmCount)
                            .tiny_unsigned()
                            .default(0),
                    )
                    .col(ColumnDef::new(Column::Nonce).big_unsigned())
                    .col(ColumnDef::new(Column::GasLimit).big_unsigned())
                    .col(ColumnDef::new(Column::GasUsed).big_unsigned())
                    .col(ColumnDef::new(Column::BlockNumber).big_unsigned())
                    .col(ColumnDef::new(Column::CreatedAt).timestamp())
                    .col(ColumnDef::new(Column::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
