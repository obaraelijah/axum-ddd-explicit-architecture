use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tracing::instrument(name = "handle_debug", skip())]
pub async fn handle_debug() -> impl IntoResponse {
    tracing::info!("info");
    tracing::error!("error");
    tracing::warn!("warn");
    tracing::debug!("debug");
    tracing::trace!("trace");
    (StatusCode::OK).into_response()
}
