use crate::{models::response_models::MessageResponse, resources};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title="HTTP Profiling",
        description="A small web-service which sole purpose is to serve content to be requested with different HTTP versions to profile various HTTP metrics.\n\nAll available docs: Rapidoc (/docs), Swagger (/swagger) and Redoc (/redoc).\n\nIf you find bugs or have feedback please create an issue here: https://github.com/Zitronenjoghurt/http-profiling-api/issues"
    ),
    paths(
        resources::ping::get_ping,
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints"),
    ),
    components(
        schemas(MessageResponse),
    )
)]
pub struct ApiDoc;
