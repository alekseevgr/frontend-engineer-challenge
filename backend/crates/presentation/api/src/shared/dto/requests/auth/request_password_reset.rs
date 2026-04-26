use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestPasswordResetRequest {
    /// Email address to send password reset instructions
    #[validate(email)]
    #[schema(example = "user@example.com", format = "email")]
    pub email: String,
}

impl From<RequestPasswordResetRequest>
    for application::dto::requests::auth::request_password_reset::RequestPasswordResetDto
{
    fn from(value: RequestPasswordResetRequest) -> Self {
        Self { email: value.email }
    }
}
