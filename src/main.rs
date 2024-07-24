#![allow(unused)]

use axum::{
    response::Html, routing::get, Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("192.168.86.84:3000")
    .await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler () -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}
