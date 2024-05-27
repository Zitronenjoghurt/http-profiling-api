use axum::Router;
use tokio::io;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod docs;

pub mod models {
    pub mod response_models;
}

pub mod resources {
    pub mod ping;
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new()
        .nest("/", resources::ping::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
