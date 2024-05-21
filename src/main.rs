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