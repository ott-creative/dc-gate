use poem::{error::ResponseError, http::StatusCode, Body, Response};
use poem_openapi::{
    payload::Json,
    registry::{MetaResponses, Registry},
    ApiResponse,
};
use serde_json::{json, Value};

#[derive(Debug, thiserror::Error)]
pub enum GateError {
    #[error(transparent)]
    EthProviderError(#[from] ethers::providers::ProviderError),
    #[error(transparent)]
    SeaDbError(#[from] sea_orm::DbErr),

    #[error("param value is required")]
    ParamValueRequired,
}

pub type Result = std::result::Result<Json<Value>, GateError>;

impl ResponseError for GateError {
    fn status(&self) -> StatusCode {
        StatusCode::OK
    }

    fn as_response(&self) -> Response {
        let body = Body::from_json(json!({
            "code": -1,
            "message": self.to_string(),
        }))
        .unwrap();
        Response::builder().status(self.status()).body(body)
    }
}

impl ApiResponse for GateError {
    fn meta() -> MetaResponses {
        MetaResponses { responses: vec![] }
    }

    fn register(_registry: &mut Registry) {}
}

pub fn ok_resp(data: Value) -> Result {
    Ok(Json(json!({
        "code": 0,
        "msg": "OK".to_string(),
        "data": data
    })))
}
