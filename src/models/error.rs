use actix_web::HttpResponse;
use serde::Serialize;

use super::envelope::ApiResponse;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    pub code: String,
    pub short_message: String,
    pub detail_message: String,
}

impl ErrorDetail {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            short_message: message.to_string(),
            detail_message: message.to_string(),
        }
    }

    pub fn to_response(&self) -> HttpResponse {
        let status = match self.code.as_str() {
            "Y300" | "Y301" | "Y302" | "Y303" | "Y304" => {
                actix_web::http::StatusCode::UNAUTHORIZED
            }
            "Y230" => actix_web::http::StatusCode::UNAUTHORIZED,
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        };
        HttpResponse::build(status).json(ApiResponse::<serde_json::Value>::error(self.clone()))
    }
}

// OTP errors
pub const Y101: &str = "Y101";
pub const Y102: &str = "Y102";
pub const Y103: &str = "Y103";

// Data errors
pub const Y104: &str = "Y104";
pub const Y105: &str = "Y105";
pub const Y106: &str = "Y106";
pub const Y109: &str = "Y109";

// Transaction errors
pub const Y201: &str = "Y201";
pub const Y212: &str = "Y212";
pub const Y217: &str = "Y217";
pub const Y223: &str = "Y223";
pub const Y226: &str = "Y226";
pub const Y227: &str = "Y227";

// Security errors
pub const Y230: &str = "Y230";

// Registration errors
pub const Y503: &str = "Y503";
pub const Y508: &str = "Y508";

// Card errors
pub const Y3117: &str = "Y3117";
pub const Y3122: &str = "Y3122";
pub const Y3125: &str = "Y3125";
pub const Y3126: &str = "Y3126";

pub fn missing_field(field: &str) -> ErrorDetail {
    ErrorDetail::new(Y105, &format!("Mandatory field {} is missing", field))
}

pub fn entity_not_found(entity_id: &str) -> ErrorDetail {
    ErrorDetail::new(Y226, &format!("Entity Id Invalid: {}", entity_id))
}

pub fn insufficient_balance() -> ErrorDetail {
    ErrorDetail::new(Y212, "Insufficient Balance")
}

pub fn invalid_otp() -> ErrorDetail {
    ErrorDetail::new(Y103, "Invalid OTP")
}

pub fn otp_expired() -> ErrorDetail {
    ErrorDetail::new(Y101, "OTP expired please regenerate OTP")
}

pub fn customer_already_registered() -> ErrorDetail {
    ErrorDetail::new(Y503, "Customer Already Registered")
}

pub fn kit_not_found() -> ErrorDetail {
    ErrorDetail::new(Y3126, "Kit not found")
}

pub fn invalid_flag() -> ErrorDetail {
    ErrorDetail::new(Y3125, "Invalid flag! Valid flags 'UL', 'L', 'BL'")
}

pub fn block_old_card() -> ErrorDetail {
    ErrorDetail::new(Y3117, "Block old card")
}

pub fn auth_failed() -> ErrorDetail {
    ErrorDetail::new(Y230, "Authentication Failed")
}

pub fn no_data() -> ErrorDetail {
    ErrorDetail::new(Y508, "No data exists")
}

pub fn invalid_txn_id(id: &str) -> ErrorDetail {
    ErrorDetail::new(Y223, &format!("Transaction ID {} is invalid", id))
}

pub fn kit_entity_mismatch() -> ErrorDetail {
    ErrorDetail::new(Y3122, "EntityId to kitNo mismatch")
}
