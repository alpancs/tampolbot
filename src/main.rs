mod webhook_telegram;

use std::env;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let app = Router::new().route(
        &format!("/webhook/telegram/{}", telegram_bot_token),
        post(webhook_telegram::handle_post),
    );
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
