use poem_openapi::{payload::PlainText, OpenApi};

pub mod eth;
pub mod uniform_response;

pub struct Api;

#[OpenApi]
impl Api {
    /// root
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("OTT PAY HK")
    }
}
