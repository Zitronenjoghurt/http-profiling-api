use crate::models::response_models::MessageResponse;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use rand::Rng;

/// Ping the API for a response.
///
/// This endpoint returns a simple pong message to indicate that the API is responsive.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Random 1MB payload", body = MessageResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
#[allow(deprecated)]
async fn get_ping() -> Response {
    let mut rng = rand::thread_rng();
    let payload: Vec<u8> = (0..1024 * 1024).map(|_| rng.gen()).collect();
    let base64_payload = base64::encode(payload);

    let response = MessageResponse {
        message: base64_payload,
    };

    println!("Request received: PING with 1MB payload");
    Json(response).into_response()
}

pub fn router() -> Router {
    Router::new().route("/", get(get_ping))
}
