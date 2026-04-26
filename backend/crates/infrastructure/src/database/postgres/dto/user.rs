use chrono::{DateTime, Utc};
use domain::{
    shared::value_objects::password_hash::PasswordHash,
    user::{
        User,
        value_objects::{email::Email, id::UserId},
    },
};
use sqlx::{FromRow, types::Uuid};

#[derive(Debug, FromRow)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        User::new(
            UserId::new(dto.id),
            Email::new(dto.email),
            PasswordHash::new(dto.password_hash),
            dto.created_at,
            dto.updated_at,
        )
    }
}
