use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::web;
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
    animation: &'static str,
}

pub async fn handle_post(update: web::Json<Update>) -> web::Json<TelegramResponse> {
    match update.message {
        Some(ref message) => handle_message(message),
        _ => web::Json(Default::default()),
    }
}

fn handle_message(message: &Message) -> web::Json<TelegramResponse> {
    match message {
        Message {
            text: Some(text), ..
        } if text.contains("@tampolbot") => web::Json(TelegramResponse {
            method: "sendAnimation",
            chat_id: message.chat.id,
            reply_to_message_id: get_reply_msg_id(message),
            animation: get_random_slap(),
        }),
        _ => web::Json(Default::default()),
    }
}

fn get_reply_msg_id(message: &Message) -> i64 {
    match message.reply_to_message {
        None => message.message_id,
        Some(ref reply_message) => reply_message.message_id,
    }
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
