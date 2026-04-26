use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    /// Password reset token received via email
    #[schema(example = "6b721a76-b1f8-4b0f-a07f-14f2d0792608")]
    pub token: String,
    /// New password for the user account (minimum 4 characters, maximum 30 characters)
    #[validate(length(min = 4, max = 30))]
    #[schema(example = "NewPassword456", min_length = 4, max_length = 30)]
    pub new_password: String,
}

impl From<ResetPasswordRequest>
    for application::dto::requests::auth::reset_password::ResetPasswordDto
{
    fn from(value: ResetPasswordRequest) -> Self {
        Self {
            token: value.token,
            new_password: value.new_password,
        }
    }
}
