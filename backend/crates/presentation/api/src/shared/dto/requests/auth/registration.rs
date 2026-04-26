use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserRequest {
    /// Email address for the new user account
    #[validate(email)]
    #[schema(example = "user@example.com", format = "email")]
    pub email: String,
    /// Password for the new user account (minimum 4 characters, maximum 30 characters)
    #[validate(length(min = 4, max = 30))]
    #[schema(example = "SecurePassword123", min_length = 4, max_length = 30)]
    pub password: String,
}

impl From<RegisterUserRequest>
    for application::dto::requests::auth::register_user::RegisterUserRequest
{
    fn from(value: RegisterUserRequest) -> Self {
        Self {
            email: value.email,
            password: value.password,
        }
    }
}
