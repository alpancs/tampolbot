use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

async fn webhook_telegram() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(8080);

    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN");
    HttpServer::new(move || {
        App::new().route(
            &format!("/webhook/telegram/{}", telegram_bot_token),
            web::post().to(webhook_telegram),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
