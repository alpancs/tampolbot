use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
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
    text: Option<String>,
}

#[derive(Deserialize)]
struct Chat {
    id: i64,
}

#[derive(Serialize, Default)]
pub struct TelegramResponse {
    method: &'static str,
    chat_id: i64,
    reply_to_message_id: i64,
    animation: Option<&'static str>,
    text: Option<&'static str>,
}

pub async fn handle_post(update: Json<Update>) -> Json<TelegramResponse> {
    match update.message.as_ref() {
        Some(message) => handle_message(message),
        _ => Default::default(),
    }
}

fn handle_message(message: &Message) -> Json<TelegramResponse> {
    match (message.text.as_ref(), message.reply_to_message.as_ref()) {
        (Some(text), Some(reply_msg)) if triggering_tampol(text) => Json(TelegramResponse {
            method: "sendAnimation",
            chat_id: message.chat.id,
            reply_to_message_id: reply_msg.message_id,
            animation: Some(get_random_slap()),
            text: None,
        }),
        (Some(text), None) if triggering_tampol(text) => Json(TelegramResponse {
            method: "sendMessage",
            chat_id: message.chat.id,
            reply_to_message_id: message.message_id,
            animation: None,
            text: Some("mau nampol pesan yang mana?"),
        }),
        _ => Default::default(),
    }
}

fn triggering_tampol(text: &str) -> bool {
    text.contains("@tampolbot") || text.contains("/tampol")
}

fn get_random_slap() -> &'static str {
    let index = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as usize
        % SLAPS.len();
    SLAPS[index]
}

const SLAPS: &[&str] = &[
    "https://thumbs.gfycat.com/WebbedSophisticatedChamois-size_restricted.gif",
    "https://c.tenor.com/KkRKZe-5sg0AAAAC/teacher-slap.gif",
    "https://c.tenor.com/d6kypk5bBL0AAAAC/slap-teddy-bear-stewie.gif",
    "https://c.tenor.com/hIMmJnbaNBwAAAAM/slap-sanmanassullavarkku-samadhanam.gif",
    "https://media.giphy.com/media/3XlEk2RxPS1m8/giphy.gif",
    "https://media.giphy.com/media/IYcXTDme1ZPMI/giphy.gif",
];
