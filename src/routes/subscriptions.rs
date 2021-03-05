use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscriberData {
    _email: String,
    _name: String,
}

pub async fn subscribe(_form: web::Form<SubscriberData>) -> impl Responder {
    HttpResponse::Ok()
}
