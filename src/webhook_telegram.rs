use actix_web::{HttpResponse, Responder};

pub async fn handle_post() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
