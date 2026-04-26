use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;

use crate::{
    shared::{repository::error::RepositoryError, value_objects::password_hash::PasswordHash},
    user::{
        User,
        value_objects::{email::Email, id::UserId},
    },
};

#[async_trait]
#[automock]
pub trait UserRepository {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError>;
    async fn get_password_hash_by_email(
        &self,
        email: &Email,
    ) -> Result<PasswordHash, RepositoryError>;
    async fn get_user_id_by_email(&self, email: &Email) -> Result<UserId, RepositoryError>;

    async fn save_password_reset_token(
        &self,
        email: &Email,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), RepositoryError>;

    async fn get_user_id_and_expiry_by_password_reset_token(
        &self,
        token: &str,
    ) -> Result<(UserId, DateTime<Utc>), RepositoryError>;

    async fn invalidate_password_reset_token(&self, token: &str) -> Result<(), RepositoryError>;

    async fn update_password(
        &self,
        user_id: &UserId,
        password_hash: &PasswordHash,
    ) -> Result<(), RepositoryError>;
}
