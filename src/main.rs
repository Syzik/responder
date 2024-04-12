use std::env;

use axum::handler::HandlerWithoutStateExt;
use responder::index;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    println!("Listening on {}...", listener.local_addr().unwrap());

    axum::serve(listener, tower::make::Shared::new(index.into_service()))
        .await
        .unwrap();
}
