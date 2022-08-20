use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Update {
    #[serde(default)]
    message: Message,
}

#[derive(Deserialize, Default)]
struct Message {
    chat: Chat,
    text: String,
}

#[derive(Deserialize, Default)]
struct Chat {
    id: i64,
}

#[derive(Serialize)]
struct TelegramResponse {
    method: &'static str,
    chat_id: i64,
    text: String,
}

pub async fn handle_post(update: web::Json<Update>) -> impl Responder {
    web::Json(TelegramResponse {
        method: "sendMessage",
        chat_id: update.message.chat.id,
        text: update.message.text.to_string(),
    })
}
