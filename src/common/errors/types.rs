use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        ApiResponse {
            success: true,
            message: message.to_string(),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str, error_detail: &str) -> Self {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
            error: Some(error_detail.to_string()),
        }
    }
}