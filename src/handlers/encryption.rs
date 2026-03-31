use actix_web::{web, HttpResponse};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EncryptRequest {
    #[serde(rename = "jsonData")]
    pub json_data: String,
    #[serde(rename = "entityKey")]
    pub entity_key: String,
}

pub async fn encrypt_with_key(form: web::Form<EncryptRequest>) -> HttpResponse {
    let encoded = general_purpose::STANDARD.encode(form.json_data.as_bytes());
    HttpResponse::Ok().content_type("text/plain").body(encoded)
}
