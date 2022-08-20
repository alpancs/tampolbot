use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Update {
    message: Option<Message>,
}

#[derive(Deserialize)]
struct Message {
    message_id: i64,
    chat: Chat,
    reply_to_message: Option<Box<Message>>,
    text: String,
}

#[derive(Deserialize)]
struct Chat {
    id: i64,
}

#[derive(Serialize, Default)]
struct TelegramResponse {
    method: &'static str,
    chat_id: i64,
    animation: &'static str,
    reply_to_message_id: i64,
}

pub async fn handle_post(update: web::Json<Update>) -> impl Responder {
    match &update.message {
        Some(message) if message.text.contains("@tampolbot") => match &message.reply_to_message {
            None => web::Json(TelegramResponse {
                method: "sendAnimation",
                chat_id: message.chat.id,
                animation: "https://tenor.com/view/teacher-slap-gif-5411272",
                reply_to_message_id: message.message_id,
            }),
            Some(reply_message) => web::Json(TelegramResponse {
                method: "sendAnimation",
                chat_id: message.chat.id,
                animation: "https://tenor.com/view/teacher-slap-gif-5411272",
                reply_to_message_id: reply_message.message_id,
            }),
        },
        _ => web::Json(Default::default()),
    }
}
