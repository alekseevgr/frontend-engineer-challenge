pub mod auth;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[schema(title = "ApiResponse")]
pub struct ApiResponse<T> {
    /// Indicates whether the request was successful
    #[schema(example = true)]
    pub success: bool,
    /// Response data
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}
