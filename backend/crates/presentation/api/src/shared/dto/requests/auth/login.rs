use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserRequest {
    /// Email address of the user
    #[validate(email)]
    #[schema(example = "user@example.com", format = "email")]
    pub email: String,
    /// User password
    #[validate(length(min = 4, max = 30))]
    #[schema(example = "SecurePassword123", min_length = 4, max_length = 30)]
    pub password: String,
}

impl From<LoginUserRequest> for application::dto::requests::auth::login_user::LoginUserRequest {
    fn from(value: LoginUserRequest) -> Self {
        Self {
            email: value.email,
            password: value.password,
        }
    }
}
