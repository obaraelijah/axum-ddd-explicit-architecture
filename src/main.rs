use crate::{
    config::connect::connect,
    handler::{handle_create_circle, handle_fetch_circle, handle_update_circle},
};

use axum::{
    routing::{get, post, put},
    Router,
};
use handler::{handle_debug, handle_get_test, handle_get_version};
use infrastructure::circle_repository_with_my_sql::CircleRepositoryWithMySql;

mod config;
mod handler;
mod tests;

#[derive(Clone)]
struct AppState {
    circle_repository: CircleRepositoryWithMySql,
    pool: sqlx::MySqlPool,
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_version))
        .route("/circle:id", get(handle_fetch_circle))
        .route("/circle/:id", put(handle_update_circle))
        .route("/circle", post(handle_create_circle))
        .route("/test", get(handle_get_test))
        .route("/debug", get(handle_debug))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let pool = connect().await.expect("database should connect");

    let state = AppState {
        circle_repository: CircleRepositoryWithMySql::new(pool.clone()),
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

