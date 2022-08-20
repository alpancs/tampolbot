use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Update {
    #[serde(default)]
    message: Message,
}

#[derive(Deserialize, Default)]
struct Message {
    message_id: i64,
    chat: Chat,
    reply_to_message: Box<Message>,
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
    reply_to_message_id: i64,
}

pub async fn handle_post(update: web::Json<Update>) -> impl Responder {
    if update.message.reply_to_message.message_id == 0 {
        web::Json(TelegramResponse {
            method: "sendMessage",
            chat_id: update.message.chat.id,
            text: update.message.text.to_string(),
            reply_to_message_id: update.message.message_id,
        })
    } else {
        web::Json(TelegramResponse {
            method: "sendMessage",
            chat_id: update.message.chat.id,
            text: update.message.reply_to_message.text.to_string(),
            reply_to_message_id: update.message.reply_to_message.message_id,
        })
    }
}
