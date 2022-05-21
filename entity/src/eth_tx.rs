use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

// TODO 金额全用decimal
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "eth_txs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: Uuid,
    pub from: String,
    pub to: String,
    pub value: u64,
    pub gas_price: Option<u64>,
    pub tx_hash: Option<String>,
    #[sea_orm(default_value = 0)]
    pub confirm_count: u8,
    pub nonce: Option<u64>,
    pub gas_limit: Option<u64>,
    pub gas_used: Option<u64>,
    pub block_number: Option<u64>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        if insert {
            self.created_at = Set(chrono::Utc::now());
        };
        self.updated_at = Set(chrono::Utc::now());
        Ok(self)
    }
}

impl Entity {
    pub async fn save_basic_tx(
        from: String,
        to: String,
        value: u64,
        db: &DatabaseConnection,
    ) -> Result<Model, DbErr> {
        ActiveModel {
            uuid: Set(Uuid::new_v4()),
            from: Set(from),
            to: Set(to),
            value: Set(value),
            ..ActiveModelTrait::default()
        }
        .insert(db)
        .await
    }
}
