use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionForm {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<SubscriptionForm>) -> HttpResponse {
    if form.name.is_empty() || form.email.is_empty() {
        HttpResponse::BadRequest().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
