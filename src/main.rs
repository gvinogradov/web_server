
use std::net::ToSocketAddrs;
use axum::{routing::get, Json, Router};
use crate::config::app_config::Config;

mod config {
    pub mod app_config;
}


#[tokio::main]
async fn main() {
    let cfg = Config::from_env();

    let address = format!("{}:{}", cfg.ipaddr, cfg.port);
    let socket_address = address.to_socket_addrs().unwrap().next().unwrap();
    let app = Router::new().route("/", get(handler));
    println!("🚀 Server started successfully on address: {address}");
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("hello world!"),
    })
}