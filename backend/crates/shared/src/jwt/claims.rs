use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub id: Uuid,
    pub exp: usize,
    pub token_type: TokenType,
}

impl Claims {
    pub fn new(id: Uuid, exp: usize, token_type: TokenType) -> Self {
        Self {
            id,
            exp,
            token_type,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    /// JWT access token for API authentication
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    /// JWT refresh token for obtaining new access tokens
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
    /// Token type (always "Bearer")
    #[schema(example = "Bearer")]
    pub token_type: String,
    /// Access token expiration time in seconds
    #[schema(example = 900)]
    pub expires_in: i64,
}
