use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use usecase::create_circle::{CreateCircleInput, CreateCircleOutput};

pub async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleRequestBody {
    pub circle_name: String,
    pub capacity: i16,
    pub owner_name: String,
    pub owner_age: i16,
    pub owner_grade: i16,
    pub owner_major: String,
}

impl std::convert::From<CreateCircleRequestBody> for CreateCircleInput {
    fn from(CreateCircleRequestBody {
        circle_name,
        capacity,
        owner_name,
        owner_age,
        owner_grade,
        owner_major,
    }: CreateCircleRequestBody,
    ) -> Self {
        CreateCircleInput::new(
            circle_name, 
            capacity, 
            owner_name, 
            owner_age, 
            owner_grade, 
            owner_major
        )
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleResponseBody {
    pub circle_id: i16,
    pub owner_id: i16,
}

impl std::convert::From<CreateCircleOutput> for CreateCircleResponseBody {
    fn from(
        CreateCircleOutput {
            circle_id,
            owner_id,
        }: CreateCircleOutput,
    ) -> Self {
        CreateCircleResponseBody {
            circle_id,
            owner_id,
        }
    }
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
