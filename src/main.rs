use axum::handler::HandlerWithoutStateExt;
use responder::index;

const BIND: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind(BIND).await.unwrap();
    println!("Listening on {}...", listener.local_addr().unwrap());

    axum::serve(listener, tower::make::Shared::new(index.into_service()))
        .await
        .unwrap();
}
