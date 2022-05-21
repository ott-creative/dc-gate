use dc_gate::{eth::EthApi, Api};
use migration::{Migrator, MigratorTrait};
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let port = env::var("PORT").expect("PORT is not set in .env file");

    let api_service = OpenApiService::new((Api, EthApi), "DC Gate", "1.0")
        .server(format!("http://localhost:{}", port));
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .data(conn);

    Server::new(TcpListener::bind(format!("127.0.0.1:{}", port)))
        .run(app)
        .await
        .unwrap();
}
