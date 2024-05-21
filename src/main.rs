use axum::{routing::get, Router};

use crate::config::connect::connect;

mod config;

#[derive(Clone)]
struct AppState {
    pool: sqlx::MySqlPool,
}
fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_version))
}

#[tokio::main]
async fn main() -> Result<(), ()>{
    let pool = connect().await.expect("database should connect");

    let state = AppState {
        pool,
    };

    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}


async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::connect::connect_test;

    use axum::http::StatusCode;
    use tower::ServiceExt;

    async fn test_version() -> anyhow::Result<()> {
        let pool = connect_test().await.expect("database should connect");
        let state = AppState {
            pool,
        };
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