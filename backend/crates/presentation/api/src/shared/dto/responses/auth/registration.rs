use chrono::{DateTime, Utc};
use domain::user::User;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserResponse {
    /// Unique identifier of the created user
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    /// Email address of the user
    #[schema(example = "user@example.com")]
    pub email: String,
    /// Timestamp when the user was created
    #[schema(example = "2024-03-27T10:30:00Z")]
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user was last updated
    #[schema(example = "2024-03-27T10:30:00Z")]
    pub updated_at: DateTime<Utc>,
}

impl From<User> for RegisterUserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id().uuid(),
            email: value.email().as_str().to_owned(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}
