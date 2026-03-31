use serde::Serialize;

use super::error::ErrorDetail;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub result: Option<T>,
    pub exception: Option<ErrorDetail>,
    pub pagination: Option<serde_json::Value>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            result: Some(data),
            exception: None,
            pagination: None,
        }
    }

    pub fn error(detail: ErrorDetail) -> ApiResponse<serde_json::Value> {
        ApiResponse {
            result: None,
            exception: Some(detail),
            pagination: None,
        }
    }
}
