use axum::{routing::get, Router};

use crate::config::connect::connect;
use handler::{handle_debug, handle_get_version};

mod config;
mod handler;

#[derive(Clone)]
struct AppState {
    pool: sqlx::MySqlPool,
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_version))
        .route("/debug", get(handle_debug))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let pool = connect().await.expect("database should connect");

    let state = AppState { pool };

    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::connect::connect_test;

    use axum::http::StatusCode;
    use tower::ServiceExt;

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_version() -> anyhow::Result<()> {
        let pool = connect_test().await.expect("database should connect");
        let state = AppState { pool };
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, "0.1.0");
        Ok(())
    }
}
