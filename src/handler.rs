use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::Row;
use std::env;
use usecase::{
    create_circle::{CreateCircleInput, CreateCircleOutput, CreateCircleUsecase},
    fetch_circle::{FetchCircleInput, FetchCircleOutput, FetchCircleUsecase, MemberOutput},
    update_circle::{UpdateCircleInput, UpdateCircleOutPut, UpdateCircleUsecase},
};

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
    fn from(
        CreateCircleRequestBody {
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
            owner_major,
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

pub async fn handle_create_circle(
    State(state): State<AppState>,
    Json(body): Json<CreateCircleRequestBody>,
) -> Result<Json<CreateCircleResponseBody>, String> {
    let circle_circle_input = CreateCircleInput::from(body);
    let mut usecase = CreateCircleUsecase::new(state.circle_repository);
    usecase
        .execute(circle_circle_input)
        .await
        .map(CreateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: i16,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetcheCircleResponseBody {
    pub circle_id: i16,
    pub circle_name: String,
    pub capacity: i16,
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

impl std::convert::From<FetchCircleOutput> for FetcheCircleResponseBody {
    fn from(
        FetchCircleOutput {
            circle_id,
            circle_name,
            capacity,
            owner,
            members,
        }: FetchCircleOutput,
    ) -> Self {
        FetcheCircleResponseBody {
            circle_id,
            circle_name,
            capacity,
            owner,
            members,
        }
    }
}

pub async fn handle_fetch_circle(
    State(state): State<AppState>,
    Path(param): Path<FetchCircleInputParam>,
) -> Result<Json<FetcheCircleResponseBody>, String> {
    let fetch_circle_input = FetchCircleInput::new(param.id);
    let usecase = FetchCircleUsecase::new(state.circle_repository);
    usecase
        .execute(fetch_circle_input)
        .await
        .map(FetcheCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
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
