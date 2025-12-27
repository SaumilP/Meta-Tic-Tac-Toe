use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod ws;
mod redis_store;
mod elo;
mod game;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/ws", ws::router());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
