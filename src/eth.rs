use crate::uniform_response::{ok_resp, GateError, Result};
use entity::eth_tx::Entity as EthTx;
use ethers::{
    prelude::Middleware,
    providers::{Http, Provider},
};
use lazy_static::lazy_static;
use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi};
use sea_orm::{DatabaseConnection, JsonValue};
use serde_json::json;

lazy_static! {
    pub static ref PROVIDER: Provider<Http> = Provider::<Http>::try_from(
        std::env::var("ETH_PROVIDER").expect("ETH_PROVIDER is not set in .env file")
    )
    .expect("could not instantiate ETH_PROVIDER");
}

pub struct EthApi;

#[OpenApi]
impl EthApi {
    /// Get real-time gas price
    #[oai(path = "/eth/gas_price", method = "get")]
    async fn gas_price(&self) -> Result {
        let price = PROVIDER.get_gas_price().await?;
        ok_resp(json!({
          "gas_price": price.as_u64()
        }))
    }

    /// Fill transaction
    #[oai(path = "/eth/fill_tx", method = "post")]
    async fn fill_tx(&self, db: Data<&DatabaseConnection>, req: Json<JsonValue>) -> Result {
        let value = req["value"].as_u64().ok_or(GateError::ParamValueRequired)?;
        // TODO fill tx (nonce gas),hash tx
        let tx = EthTx::save_basic_tx(req["from"].to_string(), req["to"].to_string(), value, &db)
            .await?;
        ok_resp(json!({
            "request_uuid": tx.uuid
        }))
    }
}
