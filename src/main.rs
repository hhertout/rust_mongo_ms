use std::env;
use axum::Router;
use dotenv::dotenv;
use tokio::net::TcpListener;

mod router;
mod controllers;
mod config;
mod repository;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let env_mode =
        env::var("RUST_ENV").unwrap_or_else(|_| panic!("Failed to load global env variable"));
    if env_mode == "development" {
        println!("⚠️ Running in development mode 🔨🔨");
        env::set_var("RUST_BACKTRACE", "1");
    }

    let port = std::env::var("SERV_PORT").unwrap_or_else(|_| String::from("4000"));
    let uri = std::env::var("SERV_IP").unwrap_or_else(|_| String::from("127.0.0.1"));
    let address = uri.to_owned() + ":" + port.as_str();

    let app: Router = router::serve().await;
    let listener = TcpListener::bind(address)
        .await
        .unwrap();

    println!("📡 Server started ! Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
